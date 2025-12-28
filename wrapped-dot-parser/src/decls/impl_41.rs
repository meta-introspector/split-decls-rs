macro_rules! deps {
    () => {
        AttrList!();
        AList!();
    };
}

macro_rules! impl_41 {
    () => {
        deps!();
        impl < A > AttrList < A > { pub (crate) fn filter_map_attr < B > (self , f : & dyn Fn (A) -> Option < B >) -> AttrList < B > { AttrList { elems : self . into_iter () . map (| alist | alist . filter_map_attr (f)) . collect () , } } # [doc = " Flatten the nested `AList`s: returns a single `AList` that contains all"] # [doc = " attributes contained in the `AttrList`."] pub fn flatten (self) -> AList < A > { self . into () } # [doc = " Flatten the nested `AList`s: returns a single `AList` that contains all"] # [doc = " attributes contained in the `AttrList`."] pub fn flatten_ref (& self) -> AList < & A > { self . into () } }
    };
}

impl_41!()