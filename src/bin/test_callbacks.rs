include!("import_macros.rs");

// Define callback macros for testing
macro_rules! callback_poly8x8x3_t {
    () => {
        println!("🎯 Callback triggered for poly8x8x3_t!");
    };
}

macro_rules! callback_filter_map {
    () => {
        println!("🎯 Callback triggered for filter_map!");
    };
}

fn main() {
    println!("🔧 Testing callback system");
    
    // Test mkdeclfn directly
    mkdeclfn!(poly8x8x3_t);
    mkdeclfn!(filter_map);
    mkdeclfn!(nonexistent_function);
    
    println!("✅ Callback test complete");
}
