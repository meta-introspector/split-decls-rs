use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::ast_statistics::TypeManifold;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Level8DPoint {
    pub coordinates: [f64; 8],
    pub level: u8,
    pub generation: u64,
    pub cached_result: Option<BranchingStructure>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BranchingStructure {
    pub size: usize,
    pub branches: Vec<Level8DPoint>,
    pub fiber_bundle_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BottPeriodicityCache {
    pub levels: HashMap<u8, Vec<Level8DPoint>>,
    pub fiber_bundles: HashMap<String, QuasiFiberBundle>,
    pub current_generation: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuasiFiberBundle {
    pub base_point: Level8DPoint,
    pub next_8_levels: [Option<Level8DPoint>; 8],
    pub periodicity_confirmed: bool,
}

impl BottPeriodicityCache {
    pub fn new() -> Self {
        Self {
            levels: HashMap::new(),
            fiber_bundles: HashMap::new(),
            current_generation: 0,
        }
    }

    pub fn collect_8d_statistics(&mut self, manifold: &TypeManifold) -> Level8DPoint {
        let coordinates = [
            manifold.dimensions[0] as f64,
            manifold.dimensions[1] as f64,
            manifold.dimensions[2] as f64,
            manifold.dimensions[3] as f64,
            manifold.dimensions[4] as f64,
            manifold.dimensions[5] as f64,
            manifold.dimensions[6] as f64,
            manifold.dimensions[7] as f64,
        ];

        Level8DPoint {
            coordinates,
            level: 1,
            generation: self.current_generation,
            cached_result: None,
        }
    }

    pub fn create_branching_structure(&mut self, point: &Level8DPoint) -> BranchingStructure {
        let size = self.calculate_exact_size(&point.coordinates);
        let mut branches = Vec::with_capacity(size);
        
        // Generate branches based on 8D coordinates
        for i in 0..size {
            let mut new_coords = point.coordinates;
            // Transform coordinates using level-specific rules
            for j in 0..8 {
                new_coords[j] *= (1.0 + (i as f64 * 0.1)) / (point.level as f64 + 1.0);
            }
            
            branches.push(Level8DPoint {
                coordinates: new_coords,
                level: point.level + 1,
                generation: self.current_generation,
                cached_result: None,
            });
        }

        BranchingStructure {
            size,
            branches,
            fiber_bundle_id: format!("fb_{}_{}", point.level, self.current_generation),
        }
    }

    pub fn cache_result(&mut self, point: Level8DPoint, structure: BranchingStructure) {
        let level = point.level;
        
        // Cache the point with its result
        let mut cached_point = point;
        cached_point.cached_result = Some(structure.clone());
        
        self.levels.entry(level).or_insert_with(Vec::new).push(cached_point.clone());
        
        // Create quasi fiber bundle if at level N
        if level <= 8 {
            self.create_quasi_fiber_bundle(cached_point, structure);
        }
        
        // Check for Bott periodicity (level 8 -> level 1)
        if level == 8 {
            self.check_bott_periodicity(&cached_point);
        }
    }

    fn create_quasi_fiber_bundle(&mut self, base_point: Level8DPoint, structure: BranchingStructure) {
        let mut next_8_levels = [None; 8];
        
        // Generate next 8 levels from current structure
        for (i, branch) in structure.branches.iter().take(8).enumerate() {
            if let Some(cached) = &branch.cached_result {
                if !cached.branches.is_empty() {
                    next_8_levels[i] = Some(cached.branches[0].clone());
                }
            }
        }

        let bundle = QuasiFiberBundle {
            base_point: base_point.clone(),
            next_8_levels,
            periodicity_confirmed: false,
        };

        self.fiber_bundles.insert(structure.fiber_bundle_id, bundle);
    }

    fn check_bott_periodicity(&mut self, level8_point: &Level8DPoint) {
        // Assert: level 8 becomes point in level 1 again
        let level1_coords = self.reduce_to_level1(&level8_point.coordinates);
        
        let level1_point = Level8DPoint {
            coordinates: level1_coords,
            level: 1,
            generation: self.current_generation + 1,
            cached_result: None,
        };

        println!("🔄 BOTT PERIODICITY: Level 8 -> Level 1");
        println!("  L8: {:?}", level8_point.coordinates);
        println!("  L1: {:?}", level1_point.coordinates);
        
        // Update generation for next cycle
        self.current_generation += 1;
        
        // Mark periodicity in fiber bundle
        if let Some(bundle_id) = self.find_bundle_for_point(level8_point) {
            if let Some(bundle) = self.fiber_bundles.get_mut(&bundle_id) {
                bundle.periodicity_confirmed = true;
            }
        }
    }

    fn reduce_to_level1(&self, coords: &[f64; 8]) -> [f64; 8] {
        let mut reduced = [0.0; 8];
        for i in 0..8 {
            // Bott periodicity reduction: mod operation in 8D space
            reduced[i] = coords[i] % 1.0;
        }
        reduced
    }

    fn calculate_exact_size(&self, coordinates: &[f64; 8]) -> usize {
        // Calculate branching size based on 8D coordinates
        let sum: f64 = coordinates.iter().sum();
        let product: f64 = coordinates.iter().product();
        
        // Ensure size is reasonable and deterministic
        ((sum.abs() + product.abs()) % 100.0) as usize + 1
    }

    fn find_bundle_for_point(&self, point: &Level8DPoint) -> Option<String> {
        for (id, bundle) in &self.fiber_bundles {
            if self.points_similar(&bundle.base_point, point) {
                return Some(id.clone());
            }
        }
        None
    }

    fn points_similar(&self, p1: &Level8DPoint, p2: &Level8DPoint) -> bool {
        const EPSILON: f64 = 1e-6;
        p1.coordinates.iter().zip(p2.coordinates.iter())
            .all(|(a, b)| (a - b).abs() < EPSILON)
    }

    pub fn reuse_profile_at_level(&mut self, level: u8) -> Option<BranchingStructure> {
        if let Some(points) = self.levels.get(&level) {
            if let Some(point) = points.last() {
                if let Some(cached) = &point.cached_result {
                    println!("♻️  REUSING PROFILE: Level {} -> Level {}", level, level + 1);
                    return Some(cached.clone());
                }
            }
        }
        None
    }

    pub fn generate_next_level(&mut self, current_level: u8) -> Option<Vec<Level8DPoint>> {
        if let Some(structure) = self.reuse_profile_at_level(current_level) {
            // Create level N+1 from cached structure
            let next_points: Vec<Level8DPoint> = structure.branches.into_iter()
                .map(|mut p| {
                    p.level = current_level + 1;
                    p.generation = self.current_generation;
                    p
                })
                .collect();
            
            Some(next_points)
        } else {
            None
        }
    }

    pub fn save_cache(&self, path: &str) -> anyhow::Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json)?;
        Ok(())
    }

    pub fn load_cache(path: &str) -> anyhow::Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let cache = serde_json::from_str(&json)?;
        Ok(cache)
    }
}

// Macro for easy 8D statistics collection
#[macro_export]
macro_rules! collect_8d_stats {
    ($cache:expr, $manifold:expr) => {
        {
            let point = $cache.collect_8d_statistics($manifold);
            let structure = $cache.create_branching_structure(&point);
            $cache.cache_result(point.clone(), structure.clone());
            
            println!("📊 8D STATISTICS COLLECTED:");
            println!("  Level: {}", point.level);
            println!("  Coordinates: {:?}", point.coordinates);
            println!("  Branches: {}", structure.size);
            
            (point, structure)
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bott_periodicity() {
        let mut cache = BottPeriodicityCache::new();
        
        // Create a level 8 point
        let level8_point = Level8DPoint {
            coordinates: [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0],
            level: 8,
            generation: 0,
            cached_result: None,
        };
        
        // Test periodicity reduction
        cache.check_bott_periodicity(&level8_point);
        
        assert_eq!(cache.current_generation, 1);
    }

    #[test]
    fn test_level_generation() {
        let mut cache = BottPeriodicityCache::new();
        
        // Create and cache a structure
        let point = Level8DPoint {
            coordinates: [0.5; 8],
            level: 1,
            generation: 0,
            cached_result: None,
        };
        
        let structure = cache.create_branching_structure(&point);
        cache.cache_result(point, structure);
        
        // Generate next level
        let next_level = cache.generate_next_level(1);
        assert!(next_level.is_some());
    }
}
