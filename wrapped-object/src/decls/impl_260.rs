macro_rules! deps {
    () => {
        CoffHeader!();
        CoffComdat!();
        CoffComdatIterator!();
        Item!();
        ReadRef!();
    };
}

macro_rules! impl_260 {
    () => {
        deps!();
        impl < 'data , 'file , R : ReadRef < 'data > , Coff : CoffHeader > Iterator for CoffComdatIterator < 'data , 'file , R , Coff > { type Item = CoffComdat < 'data , 'file , R , Coff > ; fn next (& mut self) -> Option < Self :: Item > { loop { let index = self . index ; let symbol = self . file . common . symbols . symbol (index) . ok () ? ; self . index . 0 += 1 + symbol . number_of_aux_symbols () as usize ; if let Some (comdat) = CoffComdat :: parse (self . file , symbol , index) { return Some (comdat) ; } } } }
    };
}

impl_260!();