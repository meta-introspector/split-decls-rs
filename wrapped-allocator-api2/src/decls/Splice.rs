macro_rules! deps {
    () => {
        Vec!();
        Allocator!();
        Drain!();
        Global!();
    };
}

macro_rules! Splice {
    () => {
        deps!();
        # [doc = " A splicing iterator for `Vec`."] # [doc = ""] # [doc = " This struct is created by [`Vec::splice()`]."] # [doc = " See its documentation for more."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use allocator_api2::vec;"] # [doc = ""] # [doc = " let mut v = vec![0, 1, 2];"] # [doc = " let new = [7, 8];"] # [doc = " let iter: vec::Splice<_> = v.splice(1.., new);"] # [doc = " ```"] # [derive (Debug)] pub struct Splice < 'a , I : Iterator + 'a , A : Allocator + 'a = Global > { pub (super) drain : Drain < 'a , I :: Item , A > , pub (super) replace_with : I , }
    };
}

Splice!()