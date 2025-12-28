macro_rules! deps {
    () => {
        SplitProducer!();
        Folder!();
        UnindexedProducer!();
    };
}

macro_rules! impl_851 {
    () => {
        deps!();
        impl < 'a , D , S > UnindexedProducer for SplitProducer < 'a , D , S > where D : Send , S : Fn (D) -> (D , Option < D >) + Sync , { type Item = D ; fn split (mut self) -> (Self , Option < Self >) { let splitter = self . splitter ; let (left , right) = splitter (self . data) ; self . data = left ; (self , right . map (| data | SplitProducer { data , splitter })) } fn fold_with < F > (self , folder : F) -> F where F : Folder < Self :: Item > , { folder . consume (self . data) } }
    };
}

impl_851!();