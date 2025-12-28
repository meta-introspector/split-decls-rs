macro_rules! TermVid {
    () => {
        # [derive (Copy , Clone , Eq , PartialEq , Debug)] enum TermVid { Ty (ty :: TyVid) , Const (ty :: ConstVid) , }
    };
}

TermVid!();