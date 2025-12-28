macro_rules! try_call {
    () => {
        macro_rules ! try_call { (raw ::$ p : ident ($ ($ e : expr) ,*)) => ({ match crate :: call :: c_try (raw ::$ p ($ (crate :: call :: convert (&$ e)) ,*)) { Ok (o) => o , Err (e) => { crate :: panic :: check () ; return Err (e) } } }) }
    };
}

try_call!()