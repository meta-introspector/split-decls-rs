use split_decls_rs::introspect_macro::{IntrospectMacro, generate_full_introspection_tower};
use syn::{parse_quote, Item};

fn main() {
    println!("🌀 INTROSPECT MACRO GENERATOR");
    println!("Generating self-aware macros across 8 Bott levels\n");

    // Generate the full 8-level introspection tower
    let tower = generate_full_introspection_tower();
    println!("Generated introspection tower:");
    println!("{}\n", tower);

    // Demo: Create introspection for different abstraction levels
    let mut introspector = IntrospectMacro::new();
    
    // Test with a sample function
    let sample_fn: Item = parse_quote! {
        fn hello_world() {
            println!("Hello, world!");
        }
    };

    println!("🎯 LEVEL-BY-LEVEL INTROSPECTION:");
    
    for level in 0..8 {
        println!("\n--- Bott Level {} ---", level);
        let macro_code = introspector.generate_introspect_macro(&sample_fn);
        println!("{}", macro_code);
        introspector.advance_level();
    }

    // Generate emergent runtime introspection
    println!("\n🌀 EMERGENT RUNTIME INTROSPECTION:");
    let emergent = introspector.generate_emergent_introspect();
    println!("{}", emergent);

    println!("\n✅ INTROSPECTION SYSTEM COMPLETE");
    println!("🎯 Key Features:");
    println!("  📊 Compile-time introspection at Level 0");
    println!("  🔍 Runtime pattern analysis at Level 1");
    println!("  🌀 Meta-analysis at Level 2");
    println!("  🔄 Quaternionic self-modification at Level 4");
    println!("  ⚡ Emergent runtime adaptation");
    println!("  🔄 Full 8-level Bott periodicity cycle");
}
