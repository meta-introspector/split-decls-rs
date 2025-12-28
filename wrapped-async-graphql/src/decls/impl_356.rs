macro_rules! deps {
    () => {
        SelectionField!();
        Result!();
    };
}

macro_rules! impl_356 {
    () => {
        deps!();
        impl Debug for SelectionField < '_ > { fn fmt (& self , f : & mut Formatter < '_ >) -> std :: fmt :: Result { struct DebugSelectionSet < 'a > (Vec < SelectionField < 'a > >) ; impl Debug for DebugSelectionSet < '_ > { fn fmt (& self , f : & mut Formatter < '_ >) -> std :: fmt :: Result { f . debug_list () . entries (& self . 0) . finish () } } f . debug_struct (self . name ()) . field ("name" , & self . name ()) . field ("selection_set" , & DebugSelectionSet (self . selection_set () . collect ()) ,) . finish () } }
    };
}

impl_356!()