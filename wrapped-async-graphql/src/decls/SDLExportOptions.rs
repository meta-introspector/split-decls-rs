macro_rules! SDLExportOptions {
    () => {
        # [doc = " Options for SDL export"] # [derive (Debug , Copy , Clone)] pub struct SDLExportOptions { sorted_fields : bool , sorted_arguments : bool , sorted_enum_values : bool , federation : bool , prefer_single_line_descriptions : bool , include_specified_by : bool , compose_directive : bool , use_space_ident : bool , indent_width : u8 , }
    };
}

SDLExportOptions!();