macro_rules! deps {
    () => {
        XcoffSymbol!();
        ReadRef!();
        Item!();
        FileHeader!();
        XcoffSymbolIterator!();
    };
}

macro_rules! impl_819 {
    () => {
        deps!();
        impl < 'data , 'file , Xcoff : FileHeader , R : ReadRef < 'data > > Iterator for XcoffSymbolIterator < 'data , 'file , Xcoff , R > { type Item = XcoffSymbol < 'data , 'file , Xcoff , R > ; fn next (& mut self) -> Option < Self :: Item > { let (index , symbol) = self . symbols . next () ? ; Some (XcoffSymbol { file : self . file , symbols : self . symbols . symbols , index , symbol , }) } }
    };
}

impl_819!()