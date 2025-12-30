// Generated macro for impl_29 (impl)
macro_rules! Depcrateimpl_29 {
() => {
// Module: crate
// Provides: {"impl_29"}
// Dependencies: {}
impl core :: iter :: Iterator for ArrayIter < '_ > { type Item = JsValue ; fn next (& mut self) -> Option < Self :: Item > { let index = self . range . next () ? ; Some (self . array . get (index)) } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { self . range . size_hint () } # [inline] fn count (self) -> usize where Self : Sized , { self . range . count () } # [inline] fn last (self) -> Option < Self :: Item > where Self : Sized , { let Self { range , array } = self ; range . last () . map (| index | array . get (index)) } # [inline] fn nth (& mut self , n : usize) -> Option < Self :: Item > { self . range . nth (n) . map (| index | self . array . get (index)) } }
};
}
