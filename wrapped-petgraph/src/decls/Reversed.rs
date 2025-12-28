macro_rules! Reversed {
    () => {
        # [doc = " An edge-reversing graph adaptor."] # [doc = ""] # [doc = " All edges have the opposite direction with `Reversed`."] # [derive (Copy , Clone , Debug)] pub struct Reversed < G > (pub G) ;
    };
}

Reversed!()