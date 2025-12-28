macro_rules! NvOffset {
    () => {
        # [doc = " A non-virtual offset, as described by the <nv-offset> production."] # [doc = ""] # [doc = " ```text"] # [doc = " <nv-offset> ::= <offset number>"] # [doc = " ```"] # [derive (Clone , Debug , PartialEq , Eq)] pub struct NvOffset (isize) ;
    };
}

NvOffset!();