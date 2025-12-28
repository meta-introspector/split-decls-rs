macro_rules! deps {
    () => {
        Folder!();
        IntersperseFolder!();
    };
}

macro_rules! impl_659 {
    () => {
        deps!();
        impl < C , T > Folder < T > for IntersperseFolder < C , T > where C : Folder < T > , T : Clone , { type Result = C :: Result ; fn consume (mut self , item : T) -> Self { if self . clone_first { self . base = self . base . consume (self . item . clone ()) ; if self . base . full () { return self ; } } else { self . clone_first = true ; } self . base = self . base . consume (item) ; self } fn consume_iter < I > (self , iter : I) -> Self where I : IntoIterator < Item = T > , { let mut clone_first = self . clone_first ; let between_item = self . item ; let base = self . base . consume_iter (iter . into_iter () . flat_map (| item | { let first = if clone_first { Some (between_item . clone ()) } else { clone_first = true ; None } ; first . into_iter () . chain (iter :: once (item)) })) ; IntersperseFolder { base , item : between_item , clone_first , } } fn complete (self) -> C :: Result { self . base . complete () } fn full (& self) -> bool { self . base . full () } }
    };
}

impl_659!()