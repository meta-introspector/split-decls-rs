macro_rules! deps {
    () => {
        Type!();
        AlternativeExprs!();
    };
}

macro_rules! impl_185 {
    () => {
        deps!();
        impl < 'db > AlternativeExprs < 'db > { # [doc = " Construct alternative trees"] # [doc = ""] # [doc = " # Arguments"] # [doc = " `threshold` - threshold value for many trees (more than that is many)"] # [doc = " `exprs` - expressions iterator"] fn new (threshold : usize , exprs : impl Iterator < Item = Expr < 'db > >) -> AlternativeExprs < 'db > { let mut it = AlternativeExprs :: Few (Default :: default ()) ; it . extend_with_threshold (threshold , exprs) ; it } # [doc = " Get type trees stored in alternative trees (or `Expr::Many` in case of many)"] # [doc = ""] # [doc = " # Arguments"] # [doc = " `ty` - Type of expressions queried (this is used to give type to `Expr::Many`)"] fn exprs (& self , ty : & Type < 'db >) -> Vec < Expr < 'db > > { match self { AlternativeExprs :: Few (exprs) => exprs . iter () . cloned () . collect () , AlternativeExprs :: Many => vec ! [Expr :: Many (ty . clone ())] , } } # [doc = " Extend alternative expressions"] # [doc = ""] # [doc = " # Arguments"] # [doc = " `threshold` - threshold value for many trees (more than that is many)"] # [doc = " `exprs` - expressions iterator"] fn extend_with_threshold (& mut self , threshold : usize , exprs : impl Iterator < Item = Expr < 'db > >) { match self { AlternativeExprs :: Few (tts) => { for it in exprs { if tts . len () > threshold { * self = AlternativeExprs :: Many ; break ; } tts . insert (it) ; } } AlternativeExprs :: Many => () , } } fn is_many (& self) -> bool { matches ! (self , AlternativeExprs :: Many) } }
    };
}

impl_185!()