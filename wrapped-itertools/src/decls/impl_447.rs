macro_rules! deps {
    () => {
        ProcessResults!();
    };
}

macro_rules! impl_447 {
    () => {
        deps!();
        impl < I , T , E > Iterator for ProcessResults < '_ , I , E > where I : Iterator < Item = Result < T , E > > , { type Item = T ; fn next (& mut self) -> Option < Self :: Item > { let item = self . iter . next () ; self . next_body (item) } fn size_hint (& self) -> (usize , Option < usize >) { (0 , self . iter . size_hint () . 1) } fn fold < B , F > (mut self , init : B , mut f : F) -> B where Self : Sized , F : FnMut (B , Self :: Item) -> B , { let error = self . error ; self . iter . try_fold (init , | acc , opt | match opt { Ok (x) => Ok (f (acc , x)) , Err (e) => { * error = Err (e) ; Err (acc) } }) . unwrap_or_else (| e | e) } }
    };
}

impl_447!()