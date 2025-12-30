use std::fs;
use anyhow::Result;

fn main() -> Result<()> {
    println!("🔍 PROOF: Testing actual macro compilation from regenerated crates");
    
    // Test 1: Include actual generated macros from wrapped-typenum
    println!("📦 Test 1: Loading typenum macros...");
    test_typenum_macros();
    
    // Test 2: Include actual generated macros from wrapped-unicode-segmentation  
    println!("📦 Test 2: Loading unicode-segmentation macros...");
    test_unicode_segmentation_macros();
    
    // Test 3: Include actual generated macros from wrapped-syn
    println!("📦 Test 3: Loading syn macros...");
    test_syn_macros();
    
    println!("✅ PROOF COMPLETE: All actual macros compiled successfully");
    
    Ok(())
}

fn test_typenum_macros() {
    // Include actual generated macro files from wrapped-typenum
    include!("../../output2/wrapped-typenum/src/depcrate_bitb0.rs");
    include!("../../output2/wrapped-typenum/src/depcrate_bitb1.rs");
    include!("../../output2/wrapped-typenum/src/depcrate_arrayaterm.rs");
    
    // Execute the macros to prove they work
    Depcrate_bitB0!();
    Depcrate_bitB1!(); 
    Depcrate_arrayATerm!();
    
    println!("  ✅ typenum macros: B0, B1, ATerm compiled and executed");
}

fn test_unicode_segmentation_macros() {
    // Include actual generated macro files from wrapped-unicode-segmentation
    include!("../../output2/wrapped-unicode-segmentation/src/depcrate_tablesUNICODE_VERSION.rs");
    
    // Execute the macro to prove it works
    Depcrate_tablesUNICODE_VERSION!();
    
    println!("  ✅ unicode-segmentation macros: tables module compiled and executed");
}

fn test_syn_macros() {
    // Include actual generated macro files from wrapped-syn
    include!("../../output2/wrapped-syn/src/depcrate_parseparse.rs");
    
    // Execute the macro to prove it works  
    Depcrate_parseparse!();
    
    println!("  ✅ syn macros: parse function compiled and executed");
}
