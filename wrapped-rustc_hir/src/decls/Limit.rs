macro_rules! Limit {
    () => {
        # [doc = " New-type wrapper around `usize` for representing limits. Ensures that comparisons against"] # [doc = " limits are consistent throughout the compiler."] # [derive (Clone , Copy , Debug , HashStable_Generic , Encodable , Decodable)] pub struct Limit (pub usize) ;
    };
}

Limit!()