macro_rules! deps {
    () => {
        DBIteratorWithThreadMode!();
        Error!();
        DBAccess!();
        KVBytes!();
        Direction!();
    };
}

macro_rules! impl_135 {
    () => {
        deps!();
        impl < D : DBAccess > Iterator for DBIteratorWithThreadMode < '_ , D > { type Item = Result < KVBytes , Error > ; fn next (& mut self) -> Option < Result < KVBytes , Error > > { if self . done { None } else if let Some ((key , value)) = self . raw . item () { let item = (Box :: from (key) , Box :: from (value)) ; match self . direction { Direction :: Forward => self . raw . next () , Direction :: Reverse => self . raw . prev () , } Some (Ok (item)) } else { self . done = true ; self . raw . status () . err () . map (Result :: Err) } } }
    };
}

impl_135!();