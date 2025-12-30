// Generated macro for impl_99 (impl)
macro_rules! Depcrate_deflate_coreimpl_99 {
() => {
// Module: crate::deflate::core
// Provides: {"impl_99"}
// Dependencies: {}
impl LZOxide { const fn new () -> Self { LZOxide { codes : [0 ; LZ_CODE_BUF_SIZE] , code_position : 1 , flag_position : 0 , total_bytes : 0 , num_flags_left : 8 , } } fn write_code (& mut self , val : u8) { self . codes [usize :: from (self . code_position as u16)] = val ; self . code_position += 1 ; } fn init_flag (& mut self) { if self . num_flags_left == 8 { * self . get_flag () = 0 ; self . code_position -= 1 ; } else { * self . get_flag () >>= self . num_flags_left ; } } fn get_flag (& mut self) -> & mut u8 { & mut self . codes [usize :: from (self . flag_position as u16)] } fn plant_flag (& mut self) { self . flag_position = self . code_position ; self . code_position += 1 ; } fn consume_flag (& mut self) { self . num_flags_left -= 1 ; if self . num_flags_left == 0 { self . num_flags_left = 8 ; self . plant_flag () ; } } }
};
}
