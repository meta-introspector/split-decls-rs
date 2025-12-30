// Generated macro for update_information_recursively (function)
macro_rules! Depcrate_output_color_scaleupdate_information_recursively {
() => {
// Module: crate::output::color_scale
// Provides: {"update_information_recursively"}
// Dependencies: {}
fn update_information_recursively (information : & mut ColorScaleInformation , files : & [File < '_ >] , dot_filter : DotFilter , git : Option < & GitCache > , git_ignoring : bool , depth : TreeDepth , r : Option < RecurseOptions > ,) { for file in files { if information . options . age { Extremes :: update (file . created_time () . map (| x | x . and_utc () . timestamp_millis () as f32) , & mut information . created ,) ; Extremes :: update (file . modified_time () . map (| x | x . and_utc () . timestamp_millis () as f32) , & mut information . modified ,) ; Extremes :: update (file . accessed_time () . map (| x | x . and_utc () . timestamp_millis () as f32) , & mut information . accessed ,) ; Extremes :: update (file . changed_time () . map (| x | x . and_utc () . timestamp_millis () as f32) , & mut information . changed ,) ; } if information . options . size { let size = match file . size () { Size :: Some (size) => Some (size as f32) , _ => None , } ; Extremes :: update (size , & mut information . size) ; } if file . is_directory () && r . is_some_and (| x | ! x . is_too_deep (depth . 0)) && file . name != "." && file . name != ".." { match file . to_dir () { Ok (dir) => { let files : Vec < File < '_ > > = dir . files (dot_filter , git , git_ignoring , false , false) . collect () ; update_information_recursively (information , & files , dot_filter , git , git_ignoring , depth . deeper () , r ,) ; } Err (e) => trace ! ("Unable to access directory {}: {}" , file . name , e) , } } ; } }
};
}
