// Generated macro for impl_16 (impl)
macro_rules! Depcrateimpl_16 {
() => {
// Module: crate
// Provides: {"impl_16"}
// Dependencies: {}
impl < 'de , T : OptionSet > Visitor < 'de > for OptionSetVisitor < T > { type Value = T ; fn expecting (& self , f : & mut Formatter) -> fmt :: Result { f . write_str ("set of option strings") } fn visit_seq < A : SeqAccess < 'de > > (self , seq : A) -> Result < Self :: Value , A :: Error > { assert ! (T :: VARIANTS . len () == T :: NAMES . len ()) ; match self . 0 { CaseTransform :: Identity => extract_bits (seq , T :: NAMES) , _ => { let names : Vec < _ > = T :: NAMES . iter () . map (| name | self . 0 . apply (name)) . collect () ; extract_bits (seq , & names) } } } }
};
}
