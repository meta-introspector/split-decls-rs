macro_rules! deps {
    () => {
        Hir!();
    };
}

macro_rules! Repetition {
    () => {
        deps!();
        # [doc = " The high-level intermediate representation of a repetition operator."] # [doc = ""] # [doc = " A repetition operator permits the repetition of an arbitrary"] # [doc = " sub-expression."] # [derive (Clone , Debug , Eq , PartialEq)] pub struct Repetition { # [doc = " The minimum range of the repetition."] # [doc = ""] # [doc = " Note that special cases like `?`, `+` and `*` all get translated into"] # [doc = " the ranges `{0,1}`, `{1,}` and `{0,}`, respectively."] # [doc = ""] # [doc = " When `min` is zero, this expression can match the empty string"] # [doc = " regardless of what its sub-expression is."] pub min : u32 , # [doc = " The maximum range of the repetition."] # [doc = ""] # [doc = " Note that when `max` is `None`, `min` acts as a lower bound but where"] # [doc = " there is no upper bound. For something like `x{5}` where the min and"] # [doc = " max are equivalent, `min` will be set to `5` and `max` will be set to"] # [doc = " `Some(5)`."] pub max : Option < u32 > , # [doc = " Whether this repetition operator is greedy or not. A greedy operator"] # [doc = " will match as much as it can. A non-greedy operator will match as"] # [doc = " little as it can."] # [doc = ""] # [doc = " Typically, operators are greedy by default and are only non-greedy when"] # [doc = " a `?` suffix is used, e.g., `(expr)*` is greedy while `(expr)*?` is"] # [doc = " not. However, this can be inverted via the `U` \"ungreedy\" flag."] pub greedy : bool , # [doc = " The expression being repeated."] pub sub : Box < Hir > , }
    };
}

Repetition!()