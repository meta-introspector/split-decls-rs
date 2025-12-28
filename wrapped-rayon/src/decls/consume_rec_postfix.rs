macro_rules! deps {
    () => {
        Folder!();
    };
}

macro_rules! consume_rec_postfix {
    () => {
        deps!();
        fn consume_rec_postfix < F , S , B , I > (children_of : & B , s : S , mut folder : F) -> F where F : Folder < S > , B : Fn (& S) -> I , I : IntoIterator < Item = S > , { let children = (children_of) (& s) . into_iter () ; for child in children { folder = consume_rec_postfix (children_of , child , folder) ; if folder . full () { return folder ; } } folder . consume (s) }
    };
}

consume_rec_postfix!()