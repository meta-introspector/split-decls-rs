// Generated macro for impl_21 (impl)
macro_rules! Depcrateimpl_21 {
() => {
// Module: crate
// Provides: {"impl_21"}
// Dependencies: {}
impl < 'a , K : FmtConst + 'a > fmt :: Display for DisplayOrderedMap < 'a , K > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "{}::OrderedMap {{
    key: {:?},
    disps: &[" , self . path , self . state . key) ? ; for & (d1 , d2) in & self . state . disps { write ! (f , "
        ({}, {})," , d1 , d2) ? ; } write ! (f , "
    ],
    idxs: &[" ,) ? ; for & idx in & self . state . map { write ! (f , "
        {}," , idx) ? ; } write ! (f , "
    ],
    entries: &[" ,) ? ; for (key , value) in self . keys . iter () . zip (self . values . iter ()) { write ! (f , "
        ({}, {})," , Delegate (key) , value) ? ; } write ! (f , "
    ],
}}") } }
};
}
