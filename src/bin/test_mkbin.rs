include!(concat!(env!("CARGO_MANIFEST_DIR"), "/import_macros.rs"));

fn main() {
    println!("Testing mkbin macro");
    
    mkbin!();
    
    println!("mkbin test complete");
}
}
