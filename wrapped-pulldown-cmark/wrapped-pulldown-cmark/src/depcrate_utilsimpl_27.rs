// Generated macro for impl_27 (impl)
macro_rules! Depcrate_utilsimpl_27 {
() => {
// Module: crate::utils
// Provides: {"impl_27"}
// Dependencies: {}
impl < 'a , I > Iterator for DummyOffsets < I > where I : Iterator < Item = Event < 'a > > , { type Item = (Event < 'a > , Range < usize >) ; fn next (& mut self) -> Option < Self :: Item > { self . 0 . next () . map (| event | (event , 0 .. 0)) } }
};
}
