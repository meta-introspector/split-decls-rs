use std::collections::HashMap;
use anyhow::Result;

/// Monster Group order-based signature compression
/// 2^46 × 3^20 × 5^9 × 7^6 × 11^2 × 13^3 × 17 × 19 × 23 × 29 × 31 × 41 × 47 × 59 × 71
pub struct MonsterCompressor {
    /// 46 most common pairs → emoji (2^46)
    pub pair_emojis: HashMap<String, String>,
    /// 20 samples of 3-grams → emoji (3^20)  
    pub triple_emojis: HashMap<String, String>,
    /// 9 samples of 5-grams → emoji (5^9)
    pub penta_emojis: HashMap<String, String>,
    /// 6 samples of 7-grams → emoji (7^6)
    pub hepta_emojis: HashMap<String, String>,
    /// 2 samples of 11-grams → emoji (11^2)
    pub eleven_emojis: HashMap<String, String>,
    /// 3 samples of 13-grams → emoji (13^3)
    pub thirteen_emojis: HashMap<String, String>,
    /// SINGLETONS: One each for 17, 19, 23, 29, 31, 41, 47, 59, 71
    pub singleton_17: Option<String>,
    pub singleton_19: Option<String>,
    pub singleton_23: Option<String>,
    pub singleton_29: Option<String>,
    pub singleton_31: Option<String>,
    pub singleton_41: Option<String>,
    pub singleton_47: Option<String>,
    pub singleton_59: Option<String>,
    pub singleton_71: Option<String>,
    /// Frequency tracking
    pub pair_frequencies: HashMap<String, u64>,
    pub triple_frequencies: HashMap<String, u64>,
    pub penta_frequencies: HashMap<String, u64>,
    pub hepta_frequencies: HashMap<String, u64>,
    pub eleven_frequencies: HashMap<String, u64>,
    pub thirteen_frequencies: HashMap<String, u64>,
}

/// The champion signature with 71 facets (factor of 71 in Monster Group)
#[derive(Debug, Clone)]
pub struct ChampionSignature {
    pub signature: String,
    pub facets: Vec<String>,
    pub monster_order_factor: String,
}

impl MonsterCompressor {
    pub fn new() -> Self {
        Self {
            pair_emojis: HashMap::new(),
            triple_emojis: HashMap::new(),
            penta_emojis: HashMap::new(),
            hepta_emojis: HashMap::new(),
            eleven_emojis: HashMap::new(),
            thirteen_emojis: HashMap::new(),
            singleton_17: None,
            singleton_19: None,
            singleton_23: None,
            singleton_29: None,
            singleton_31: None,
            singleton_41: None,
            singleton_47: None,
            singleton_59: None,
            singleton_71: None,
            pair_frequencies: HashMap::new(),
            triple_frequencies: HashMap::new(),
            penta_frequencies: HashMap::new(),
            hepta_frequencies: HashMap::new(),
            eleven_frequencies: HashMap::new(),
            thirteen_frequencies: HashMap::new(),
        }
    }

    /// Extract n-grams from signature and apply Monster Group compression
    pub fn compress_signature(&mut self, signature: &str) -> Result<MonsterSignature> {
        let tokens: Vec<&str> = signature.split('|').collect();
        
        // Extract pairs (2-grams) - 46 most common
        let pairs = self.extract_ngrams(&tokens, 2);
        for pair in &pairs {
            *self.pair_frequencies.entry(pair.clone()).or_insert(0) += 1;
        }
        
        // Extract triples (3-grams) - 20 samples
        let triples = self.extract_ngrams(&tokens, 3);
        for triple in &triples {
            *self.triple_frequencies.entry(triple.clone()).or_insert(0) += 1;
        }
        
        // Extract 5-grams - 9 samples
        let pentas = self.extract_ngrams(&tokens, 5);
        for penta in &pentas {
            *self.penta_frequencies.entry(penta.clone()).or_insert(0) += 1;
        }
        
        // Extract 7-grams - 6 samples
        let heptas = self.extract_ngrams(&tokens, 7);
        for hepta in &heptas {
            *self.hepta_frequencies.entry(hepta.clone()).or_insert(0) += 1;
        }
        
        Ok(MonsterSignature {
            original: signature.to_string(),
            pairs,
            triples,
            pentas,
            heptas,
            compressed_form: self.generate_compressed_form(signature),
        })
    }

    /// Apply Monster Group order compression with ALL prime factors
    pub fn apply_monster_compression(&mut self) {
        // 46 most common pairs → emoji (2^46)
        let top_pairs: Vec<_> = self.get_top_n_by_frequency(&self.pair_frequencies, 46);
        for (i, (pair, _)) in top_pairs.iter().enumerate() {
            let emoji = self.get_pair_emoji(i);
            self.pair_emojis.insert(pair.clone(), emoji);
        }
        
        // 20 samples of 3-grams → emoji (3^20)
        let top_triples: Vec<_> = self.get_top_n_by_frequency(&self.triple_frequencies, 20);
        for (i, (triple, _)) in top_triples.iter().enumerate() {
            let emoji = self.get_triple_emoji(i);
            self.triple_emojis.insert(triple.clone(), emoji);
        }
        
        // 9 samples of 5-grams → emoji (5^9)
        let top_pentas: Vec<_> = self.get_top_n_by_frequency(&self.penta_frequencies, 9);
        for (i, (penta, _)) in top_pentas.iter().enumerate() {
            let emoji = self.get_penta_emoji(i);
            self.penta_emojis.insert(penta.clone(), emoji);
        }
        
        // 6 samples of 7-grams → emoji (7^6)
        let top_heptas: Vec<_> = self.get_top_n_by_frequency(&self.hepta_frequencies, 6);
        for (i, (hepta, _)) in top_heptas.iter().enumerate() {
            let emoji = self.get_hepta_emoji(i);
            self.hepta_emojis.insert(hepta.clone(), emoji);
        }
        
        // 2 samples of 11-grams → emoji (11^2)
        let top_elevens: Vec<_> = self.get_top_n_by_frequency(&self.eleven_frequencies, 2);
        for (i, (eleven, _)) in top_elevens.iter().enumerate() {
            let emoji = self.get_eleven_emoji(i);
            self.eleven_emojis.insert(eleven.clone(), emoji);
        }
        
        // 3 samples of 13-grams → emoji (13^3)
        let top_thirteens: Vec<_> = self.get_top_n_by_frequency(&self.thirteen_frequencies, 3);
        for (i, (thirteen, _)) in top_thirteens.iter().enumerate() {
            let emoji = self.get_thirteen_emoji(i);
            self.thirteen_emojis.insert(thirteen.clone(), emoji);
        }
        
        // SINGLETONS: One signature each for prime factors 17,19,23,29,31,41,47,59,71
        self.assign_singletons();
    }

    /// Assign singleton signatures for prime factors 17,19,23,29,31,41,47,59,71
    fn assign_singletons(&mut self) {
        let all_signatures = self.collect_all_unique_signatures();
        let mut sorted_by_rarity: Vec<_> = all_signatures.into_iter().collect();
        sorted_by_rarity.sort_by_key(|(_, freq)| *freq); // Rarest first
        
        // Assign the 9 rarest signatures to singleton primes
        if let Some((sig, _)) = sorted_by_rarity.get(0) {
            self.singleton_71 = Some(format!("👹{}", sig)); // 71 - rarest
        }
        if let Some((sig, _)) = sorted_by_rarity.get(1) {
            self.singleton_59 = Some(format!("🔮{}", sig)); // 59
        }
        if let Some((sig, _)) = sorted_by_rarity.get(2) {
            self.singleton_47 = Some(format!("💎{}", sig)); // 47
        }
        if let Some((sig, _)) = sorted_by_rarity.get(3) {
            self.singleton_41 = Some(format!("⚡{}", sig)); // 41
        }
        if let Some((sig, _)) = sorted_by_rarity.get(4) {
            self.singleton_31 = Some(format!("🌟{}", sig)); // 31
        }
        if let Some((sig, _)) = sorted_by_rarity.get(5) {
            self.singleton_29 = Some(format!("🔥{}", sig)); // 29
        }
        if let Some((sig, _)) = sorted_by_rarity.get(6) {
            self.singleton_23 = Some(format!("✨{}", sig)); // 23
        }
        if let Some((sig, _)) = sorted_by_rarity.get(7) {
            self.singleton_19 = Some(format!("🌈{}", sig)); // 19
        }
        if let Some((sig, _)) = sorted_by_rarity.get(8) {
            self.singleton_17 = Some(format!("🎭{}", sig)); // 17
        }
    }

    fn collect_all_unique_signatures(&self) -> HashMap<String, u64> {
        let mut all_sigs = HashMap::new();
        
        for (sig, &freq) in &self.pair_frequencies {
            *all_sigs.entry(sig.clone()).or_insert(0) += freq;
        }
        for (sig, &freq) in &self.triple_frequencies {
            *all_sigs.entry(sig.clone()).or_insert(0) += freq;
        }
        for (sig, &freq) in &self.penta_frequencies {
            *all_sigs.entry(sig.clone()).or_insert(0) += freq;
        }
        for (sig, &freq) in &self.hepta_frequencies {
            *all_sigs.entry(sig.clone()).or_insert(0) += freq;
        }
        for (sig, &freq) in &self.eleven_frequencies {
            *all_sigs.entry(sig.clone()).or_insert(0) += freq;
        }
        for (sig, &freq) in &self.thirteen_frequencies {
            *all_sigs.entry(sig.clone()).or_insert(0) += freq;
        }
        
        all_sigs
    }

    /// Find the single most common signature (the 71-facet champion)
    fn find_champion_signature(&self) -> Option<String> {
        // Combine all frequency maps to find global champion
        let mut all_frequencies = HashMap::new();
        
        for (sig, &freq) in &self.pair_frequencies {
            *all_frequencies.entry(sig.clone()).or_insert(0) += freq;
        }
        for (sig, &freq) in &self.triple_frequencies {
            *all_frequencies.entry(sig.clone()).or_insert(0) += freq;
        }
        for (sig, &freq) in &self.penta_frequencies {
            *all_frequencies.entry(sig.clone()).or_insert(0) += freq;
        }
        for (sig, &freq) in &self.hepta_frequencies {
            *all_frequencies.entry(sig.clone()).or_insert(0) += freq;
        }
        
        all_frequencies.into_iter()
            .max_by_key(|(_, freq)| *freq)
            .map(|(sig, _)| sig)
    }

    /// Create 71 facets for the champion signature
    fn create_champion_facets(&mut self, champion: String) {
        let facets = self.generate_71_facets();
        
        // Store champion with all 71 facets
        self.champion_signature = Some(ChampionSignature {
            signature: champion,
            facets,
            monster_order_factor: "71".to_string(),
        });
    }

    /// Generate 71 unique facets for the champion
    fn generate_71_facets(&self) -> Vec<String> {
        vec![
            // Mathematical facets (1-10)
            "🔢", "∞", "π", "∑", "∫", "∆", "∇", "∂", "√", "∛",
            // Geometric facets (11-20)  
            "△", "□", "○", "◇", "⬟", "⬢", "⬡", "⬠", "⬜", "⬛",
            // Algebraic facets (21-30)
            "α", "β", "γ", "δ", "ε", "ζ", "η", "θ", "λ", "μ",
            // Topological facets (31-40)
            "⊕", "⊗", "⊙", "⊚", "⊛", "⊜", "⊝", "⊞", "⊟", "⊠",
            // Group theory facets (41-50)
            "∘", "∗", "⋆", "⋄", "⋅", "⋈", "⋉", "⋊", "⋋", "⋌",
            // Category theory facets (51-60)
            "→", "↦", "⇒", "⇔", "↔", "⟶", "⟷", "⟸", "⟹", "⇄",
            // Monster group facets (61-71)
            "👹", "🐉", "🔥", "⚡", "💎", "🌟", "✨", "🌈", "🎭", "🪄", "👑"
        ]
    }

    fn extract_ngrams(&self, tokens: &[&str], n: usize) -> Vec<String> {
        if tokens.len() < n { return vec![]; }
        
        tokens.windows(n)
            .map(|window| window.join("+"))
            .collect()
    }

    fn get_top_n_by_frequency(&self, freq_map: &HashMap<String, u64>, n: usize) -> Vec<(String, u64)> {
        let mut sorted: Vec<_> = freq_map.iter().map(|(k, &v)| (k.clone(), v)).collect();
        sorted.sort_by(|a, b| b.1.cmp(&a.1));
        sorted.into_iter().take(n).collect()
    }

    /// Monster Group emoji assignments for pairs (2^46)
    fn get_pair_emoji(&self, index: usize) -> String {
        let pair_emojis = [
            "🔥", "⚡", "🌟", "💎", "🚀", "🎯", "🔧", "⚙️", "🏗️", "🎭",
            "🪄", "✨", "🌈", "🎨", "🔮", "💫", "🌙", "☀️", "⭐", "🌊",
            "🏔️", "🌋", "🌪️", "❄️", "🔥", "💥", "⚡", "🌩️", "🌈", "🎪",
            "🎡", "🎢", "🎠", "🎨", "🎭", "🎪", "🎯", "🎲", "🎰", "🃏",
            "🎴", "🀄", "🎮", "🕹️", "🎸", "🎺"
        ];
        pair_emojis.get(index).unwrap_or(&"🔹").to_string()
    }

    /// Monster Group emoji assignments for triples (3^20)
    fn get_triple_emoji(&self, index: usize) -> String {
        let triple_emojis = [
            "🔺", "🔻", "🔸", "🔹", "🔶", "🔷", "🔴", "🟠", "🟡", "🟢",
            "🔵", "🟣", "⚫", "⚪", "🟤", "🔳", "🔲", "▫️", "▪️", "◾"
        ];
        triple_emojis.get(index).unwrap_or(&"🔸").to_string()
    }

    /// Monster Group emoji assignments for 5-grams (5^9)
    fn get_penta_emoji(&self, index: usize) -> String {
        let penta_emojis = [
            "🌀", "🌊", "🌈", "🌙", "⭐", "💫", "✨", "🔮", "💎"
        ];
        penta_emojis.get(index).unwrap_or(&"🌀").to_string()
    }

    /// Monster Group emoji assignments for 7-grams (7^6)
    fn get_hepta_emoji(&self, index: usize) -> String {
        let hepta_emojis = [
            "👑", "💍", "🏆", "🎖️", "🏅", "🎗️"
        ];
        hepta_emojis.get(index).unwrap_or(&"👑").to_string()
    }

    /// Monster Group emoji assignments for 11-grams (11^2)
    fn get_eleven_emoji(&self, index: usize) -> String {
        let eleven_emojis = ["🎪", "🎡"];
        eleven_emojis.get(index).unwrap_or(&"🎪").to_string()
    }

    /// Monster Group emoji assignments for 13-grams (13^3)
    fn get_thirteen_emoji(&self, index: usize) -> String {
        let thirteen_emojis = ["🌙", "⭐", "💫"];
        thirteen_emojis.get(index).unwrap_or(&"🌙").to_string()
    }

    fn generate_compressed_form(&self, signature: &str) -> String {
        // This would generate the actual compressed representation
        // using the Monster Group factorization
        format!("M({})", signature.len())
    }
}

#[derive(Debug, Clone)]
pub struct MonsterSignature {
    pub original: String,
    pub pairs: Vec<String>,
    pub triples: Vec<String>, 
    pub pentas: Vec<String>,
    pub heptas: Vec<String>,
    pub compressed_form: String,
}

impl MonsterSignature {
    /// Calculate Monster Group compression ratio
    pub fn compression_ratio(&self) -> f64 {
        let original_size = self.original.len() as f64;
        let compressed_size = self.compressed_form.len() as f64;
        compressed_size / original_size
    }

    /// Generate Monster Group factorization
    pub fn monster_factorization(&self) -> String {
        format!("2^{} × 3^{} × 5^{} × 7^{}", 
            self.pairs.len().min(46),
            self.triples.len().min(20), 
            self.pentas.len().min(9),
            self.heptas.len().min(6)
        )
    }
}
