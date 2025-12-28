macro_rules! deps {
    () => {
        Row!();
        RowIterator!();
        AsRow!();
    };
}

macro_rules! impl_469 {
    () => {
        deps!();
        impl < R : AsRow > Iterator for RowIterator < R > { type Item = R ; fn next (& mut self) -> Option < Self :: Item > { self . rows . next () . map (| row | R :: from_row (Row :: new (self . file , row))) } }
    };
}

impl_469!();