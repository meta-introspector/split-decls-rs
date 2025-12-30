// Generated macro for skip_calls (function)
macro_rules! Depcrate_interpreterskip_calls {
() => {
// Module: crate::interpreter
// Provides: {"skip_calls"}
// Dependencies: {}
fn skip_calls (module : & Module , id : FunctionId) -> HashSet < FunctionId > { use walrus :: ir :: * ; let func = module . funcs . get (id) ; let local = match & func . kind { walrus :: FunctionKind :: Local (l) => l , _ => panic ! ("can only call locally defined functions") , } ; let entry = local . entry_block () ; let block = local . block (entry) ; block . instrs . iter () . filter_map (| (instr , _) | match instr { Instr :: Call (Call { func }) | Instr :: ReturnCall (ReturnCall { func }) => Some (* func) , _ => None , }) . collect () }
};
}
