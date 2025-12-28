macro_rules! deps {
    () => {
        Iter!();
        Error!();
    };
}

macro_rules! impl_110 {
    () => {
        deps!();
        impl Iterator for loose :: Iter { type Item = Result < gix_hash :: ObjectId , Error > ; fn next (& mut self) -> Option < Self :: Item > { while let Some (res) = self . inner . next () { if let Some (res) = self . path_to_id (res) { return Some (res) ; } } None } }
    };
}

impl_110!()