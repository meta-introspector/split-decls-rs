macro_rules! deps {
    () => {
        Sorting!();
        Queue!();
        Info!();
        Topo!();
    };
}

macro_rules! impl_23 {
    () => {
        deps!();
        impl Queue { pub (super) fn new (s : Sorting) -> Self { match s { Sorting :: DateOrder => Self :: Date (PriorityQueue :: new ()) , Sorting :: TopoOrder => Self :: Topo (vec ! []) , } } pub (super) fn push (& mut self , commit_time : i64 , info : Info) { match self { Self :: Date (q) => q . insert (commit_time , info) , Self :: Topo (q) => q . push ((commit_time , info)) , } } fn pop (& mut self) -> Option < Info > { match self { Self :: Date (q) => q . pop () . map (| (_ , info) | info) , Self :: Topo (q) => q . pop () . map (| (_ , info) | info) , } } pub (super) fn initial_sort (& mut self) { if let Self :: Topo (ref mut inner_vec) = self { inner_vec . sort_by (| a , b | a . 0 . cmp (& b . 0)) ; } } }
    };
}

impl_23!();