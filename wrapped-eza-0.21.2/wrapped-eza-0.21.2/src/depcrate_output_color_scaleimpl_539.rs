// Generated macro for impl_539 (impl)
macro_rules! Depcrate_output_color_scaleimpl_539 {
() => {
// Module: crate::output::color_scale
// Provides: {"impl_539"}
// Dependencies: {}
impl ColorScaleInformation { pub fn from_color_scale (color_scale : ColorScaleOptions , files : & [File < '_ >] , dot_filter : DotFilter , git : Option < & GitCache > , git_ignoring : bool , r : Option < RecurseOptions > ,) -> Option < Self > { if color_scale . mode == ColorScaleMode :: Fixed { None } else { let mut information = Self { options : color_scale , accessed : None , changed : None , created : None , modified : None , size : None , } ; update_information_recursively (& mut information , files , dot_filter , git , git_ignoring , TreeDepth :: root () , r ,) ; Some (information) } } pub fn adjust_style (& self , mut style : Style , value : f32 , range : Option < Extremes >) -> Style { if let (Some (fg) , Some (range)) = (style . foreground , range) { let mut ratio = ((value - range . min) / (range . max - range . min)) . clamp (0.0 , 1.0) ; if ratio . is_nan () { ratio = 1.0 ; } style . foreground = Some (adjust_luminance (fg , ratio , self . options . min_luminance as f32 / 100.0 ,)) ; } style } pub fn apply_time_gradient (& self , style : Style , file : & File < '_ > , time_type : TimeType) -> Style { let range = match time_type { TimeType :: Modified => self . modified , TimeType :: Changed => self . changed , TimeType :: Accessed => self . accessed , TimeType :: Created => self . created , } ; if let Some (file_time) = time_type . get_corresponding_time (file) { self . adjust_style (style , file_time . and_utc () . timestamp_millis () as f32 , range) } else { style } } }
};
}
