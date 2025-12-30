// Generated macro for add_pic_option (function)
macro_rules! Depcrate_baseadd_pic_option {
() => {
// Module: crate::base
// Provides: {"add_pic_option"}
// Dependencies: {}
pub fn add_pic_option < 'gcc > (context : & Context < 'gcc > , relocation_model : RelocModel) { match relocation_model { rustc_target :: spec :: RelocModel :: Static => { context . add_command_line_option ("-fno-pie") ; context . add_driver_option ("-fno-pie") ; } rustc_target :: spec :: RelocModel :: Pic => { context . add_command_line_option ("-fPIC") ; context . add_driver_option ("-fPIC") ; } rustc_target :: spec :: RelocModel :: Pie => { context . add_command_line_option ("-fPIE") ; context . add_driver_option ("-fPIE") ; } model => eprintln ! ("Unsupported relocation model: {:?}" , model) , } }
};
}
