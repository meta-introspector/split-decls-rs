macro_rules! deps {
    () => {
        Reject!();
    };
}

macro_rules! next_ch {
    () => {
        deps!();
        macro_rules ! next_ch { ($ chars : ident @ $ pat : pat) => { match $ chars . next () { Some ((_ , ch)) => match ch { $ pat => ch , _ => return Err (Reject) , } , None => return Err (Reject) , } } ; }
    };
}

next_ch!();