// Generated macro for impl_12 (impl)
macro_rules! Depcrateimpl_12 {
() => {
// Module: crate
// Provides: {"impl_12"}
// Dependencies: {}
impl < 'a , K : FmtConst + 'a > fmt :: Display for DisplayMap < 'a , K > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "{}::Map {{
    key: {:?},
    disps: &[" , self . path , self . state . key) ? ; for & (d1 , d2) in & self . state . disps { write ! (f , "
        ({}, {})," , d1 , d2) ? ; } write ! (f , "
    ],
    entries: &[" ,) ? ; for & idx in & self . state . map { write ! (f , "
        ({}, {})," , Delegate (& self . keys [idx]) , & self . values [idx]) ? ; } write ! (f , "
    ],
}}") } }
};
}
