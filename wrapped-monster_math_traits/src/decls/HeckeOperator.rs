macro_rules! deps {
    () => {
        Declaration!();
    };
}

macro_rules! HeckeOperator {
    () => {
        deps!();
        # [doc = " A trait representing a Hecke operator for applying algebraic transformations to declarations."] pub trait HeckeOperator { # [doc = " Applies an algebraic transformation (Hecke operator) to a given Declaration."] # [doc = " Returns a new, transformed Declaration."] fn apply_transformation (& self , declaration : Declaration) -> Declaration ; }
    };
}

HeckeOperator!()