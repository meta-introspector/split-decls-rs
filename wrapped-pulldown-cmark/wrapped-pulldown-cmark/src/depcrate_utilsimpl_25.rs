// Generated macro for impl_25 (impl)
macro_rules! Depcrate_utilsimpl_25 {
() => {
// Module: crate::utils
// Provides: {"impl_25"}
// Dependencies: {}
impl < 'a , I > Iterator for TextMergeStream < 'a , I > where I : Iterator < Item = Event < 'a > > , { type Item = Event < 'a > ; fn next (& mut self) -> Option < Self :: Item > { self . inner . next () . map (| (event , _) | event) } }
};
}
