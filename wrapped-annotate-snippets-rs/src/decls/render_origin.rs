macro_rules! deps {
    () => {
        Origin!();
        ElementStyle!();
        Renderer!();
        StyledBuffer!();
    };
}

macro_rules! render_origin {
    () => {
        deps!();
        # [allow (clippy :: too_many_arguments)] fn render_origin (renderer : & Renderer , buffer : & mut StyledBuffer , max_line_num_len : usize , origin : & Origin < '_ > , is_primary : bool , is_first : bool , alone : bool , buffer_msg_line_offset : usize ,) { if is_primary && ! renderer . short_message { buffer . prepend (buffer_msg_line_offset , renderer . decor_style . file_start (is_first , alone) , ElementStyle :: LineNumber ,) ; } else if ! renderer . short_message { buffer . prepend (buffer_msg_line_offset , renderer . decor_style . secondary_file_start () , ElementStyle :: LineNumber ,) ; } let str = match (& origin . line , & origin . char_column) { (Some (line) , Some (col)) => { format ! ("{}:{}:{}" , origin . path , line , col) } (Some (line) , None) => format ! ("{}:{}" , origin . path , line) , _ => origin . path . to_string () , } ; buffer . append (buffer_msg_line_offset , & str , ElementStyle :: LineAndColumn) ; if ! renderer . short_message { for _ in 0 .. max_line_num_len { buffer . prepend (buffer_msg_line_offset , " " , ElementStyle :: NoStyle) ; } } }
    };
}

render_origin!()