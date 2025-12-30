// Generated macro for Expression (struct)
macro_rules! DepcrateExpression {
() => {
// Module: crate
// Provides: {"Expression"}
// Dependencies: {}
# [doc = " An expression that can be the left hand or right hand side of a constraint equation."] # [doc = " It is a linear combination of variables, i.e. a sum of variables weighted by coefficients, plus an optional constant."] # [derive (Clone , Debug)] pub struct Expression { pub terms : Vec < Term > , pub constant : f64 }
};
}
