macro_rules! deps {
    () => {
        AsRow!();
        Item!();
        RowIterator!();
        Row!();
    };
}

macro_rules! impl_73 {
    () => {
        deps!();
        impl < 'a , R : AsRow < 'a > > Iterator for RowIterator < 'a , R > { type Item = R ; fn next (& mut self) -> Option < Self :: Item > { self . rows . next () . map (| row | R :: from_row (Row :: new (self . index , self . file , row))) } }
    };
}

impl_73!()