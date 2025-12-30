// Generated macro for show_ireg_sized (function)
macro_rules! Depcrate_isa_x64_inst_regsshow_ireg_sized {
() => {
// Module: crate::isa::x64::inst::regs
// Provides: {"show_ireg_sized"}
// Dependencies: {}
# [doc = " If `ireg` denotes an I64-classed reg, make a best-effort attempt to show its name at some"] # [doc = " smaller size (4, 2 or 1 bytes)."] pub fn show_ireg_sized (reg : Reg , size : u8) -> String { let mut s = show_reg (reg) ; if reg . class () != RegClass :: Int || size == 8 { return s ; } if reg . is_real () { let remapper = match s . as_str () { "%rax" => Some (["%eax" , "%ax" , "%al"]) , "%rbx" => Some (["%ebx" , "%bx" , "%bl"]) , "%rcx" => Some (["%ecx" , "%cx" , "%cl"]) , "%rdx" => Some (["%edx" , "%dx" , "%dl"]) , "%rsi" => Some (["%esi" , "%si" , "%sil"]) , "%rdi" => Some (["%edi" , "%di" , "%dil"]) , "%rbp" => Some (["%ebp" , "%bp" , "%bpl"]) , "%rsp" => Some (["%esp" , "%sp" , "%spl"]) , "%r8" => Some (["%r8d" , "%r8w" , "%r8b"]) , "%r9" => Some (["%r9d" , "%r9w" , "%r9b"]) , "%r10" => Some (["%r10d" , "%r10w" , "%r10b"]) , "%r11" => Some (["%r11d" , "%r11w" , "%r11b"]) , "%r12" => Some (["%r12d" , "%r12w" , "%r12b"]) , "%r13" => Some (["%r13d" , "%r13w" , "%r13b"]) , "%r14" => Some (["%r14d" , "%r14w" , "%r14b"]) , "%r15" => Some (["%r15d" , "%r15w" , "%r15b"]) , _ => None , } ; if let Some (smaller_names) = remapper { match size { 4 => s = smaller_names [0] . into () , 2 => s = smaller_names [1] . into () , 1 => s = smaller_names [2] . into () , _ => panic ! ("show_ireg_sized: real") , } } } else { let suffix = match size { 4 => "l" , 2 => "w" , 1 => "b" , _ => panic ! ("show_ireg_sized: virtual") , } ; s = s + suffix ; } s }
};
}
