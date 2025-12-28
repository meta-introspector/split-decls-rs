macro_rules! deps {
    () => {
        IndexType!();
        EdgeIndices!();
        EdgeIndex!();
    };
}

macro_rules! impl_292 {
    () => {
        deps!();
        impl < E , Ix : IndexType > Iterator for EdgeIndices < '_ , E , Ix > { type Item = EdgeIndex < Ix > ; fn next (& mut self) -> Option < EdgeIndex < Ix > > { loop { if self . cur < self . row_len { let res = self . cur ; self . cur += 1 ; return Some (EdgeIndex { from : Ix :: new (self . row_index) , successor_index : res , }) ; } else { match self . rows . next () { Some ((index , row)) => { self . row_index = index ; self . cur = 0 ; self . row_len = row . len () ; } None => return None , } } } } }
    };
}

impl_292!()