macro_rules! AmpMutSugg {
    () => {
        enum AmpMutSugg { # [doc = " Type suggestion. Changes `&self` to `&mut self`, `x: &T` to `x: &mut T`,"] # [doc = " `ref x` to `ref mut x`, etc."] Type { span : Span , suggestion : String , additional : Option < (Span , String) > , } , # [doc = " Suggestion for expressions, `&x` to `&mut x`, `&x[i]` to `&mut x[i]`, etc."] Expr { span : Span , suggestion : String , } , # [doc = " Suggests `.get_mut` in the case of `&map[&key]` for Hash/BTreeMap."] MapGetMut { span : Span , suggestion : String , } , ChangeBinding , }
    };
}

AmpMutSugg!()