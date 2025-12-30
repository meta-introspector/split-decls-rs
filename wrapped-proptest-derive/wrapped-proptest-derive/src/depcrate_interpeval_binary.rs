// Generated macro for eval_binary (function)
macro_rules! Depcrate_interpeval_binary {
() => {
// Module: crate::interp
// Provides: {"eval_binary"}
// Dependencies: {}
# [doc = " Interprets a binary operator on two expressions."] fn eval_binary (bin : & syn :: ExprBinary) -> Option < u128 > { use std :: u32 ; let l = eval_expr (& bin . left) ? ; let r = eval_expr (& bin . right) ? ; Some (match bin . op { B :: Add (_) => l . checked_add (r) ? , B :: Sub (_) => l . checked_sub (r) ? , B :: Mul (_) => l . checked_mul (r) ? , B :: Div (_) => l . checked_div (r) ? , B :: Rem (_) => l . checked_rem (r) ? , B :: BitXor (_) => l ^ r , B :: BitAnd (_) => l & r , B :: BitOr (_) => l | r , B :: Shl (_) if r <= u128 :: from (u32 :: MAX) => l . checked_shl (r as u32) ? , B :: Shr (_) if r <= u128 :: from (u32 :: MAX) => l . checked_shr (r as u32) ? , _ => return None , }) }
};
}
