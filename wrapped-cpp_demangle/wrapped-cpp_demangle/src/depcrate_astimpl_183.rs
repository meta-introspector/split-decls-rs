// Generated macro for impl_183 (impl)
macro_rules! Depcrate_astimpl_183 {
() => {
// Module: crate::ast
// Provides: {"impl_183"}
// Dependencies: {}
impl Parse for CvQualifiers { fn parse < 'a , 'b > (ctx : & 'a ParseContext , _subs : & 'a mut SubstitutionTable , input : IndexStr < 'b > ,) -> Result < (CvQualifiers , IndexStr < 'b >) > { try_begin_parse ! ("CvQualifiers" , ctx , input) ; let (restrict , tail) = if let Ok (tail) = consume (b"r" , input) { (true , tail) } else { (false , input) } ; let (volatile , tail) = if let Ok (tail) = consume (b"V" , tail) { (true , tail) } else { (false , tail) } ; let (const_ , tail) = if let Ok (tail) = consume (b"K" , tail) { (true , tail) } else { (false , tail) } ; let qualifiers = CvQualifiers { restrict : restrict , volatile : volatile , const_ : const_ , } ; Ok ((qualifiers , tail)) } }
};
}
