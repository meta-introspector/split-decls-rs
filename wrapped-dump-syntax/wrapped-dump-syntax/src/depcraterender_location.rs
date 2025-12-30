// Generated macro for render_location (function)
macro_rules! Depcraterender_location {
() => {
// Module: crate
// Provides: {"render_location"}
// Dependencies: {}
fn render_location (formatter : & mut fmt :: Formatter , err : & syn :: Error , filepath : & Path , code : & str ,) -> fmt :: Result { let start = err . span () . start () ; let mut end = err . span () . end () ; let code_line = match start . line . checked_sub (1) . and_then (| n | code . lines () . nth (n)) { Some (line) => line , None => return render_fallback (formatter , err) , } ; if end . line > start . line { end . line = start . line ; end . column = code_line . len () ; } let filename = filepath . file_name () . map (OsStr :: to_string_lossy) . unwrap_or (Cow :: Borrowed ("main.rs")) ; write ! (formatter , "\n\
         {error}{header}\n\
         {indent}{arrow} {filename}:{linenum}:{colnum}\n\
         {indent} {pipe}\n\
         {label} {pipe} {code}\n\
         {indent} {pipe} {offset}{underline} {message}\n\
         " , error = "error" . red () . bold () , header = ": Syn unable to parse file" . bold () , indent = " " . repeat (start . line . to_string () . len ()) , arrow = "-->" . blue () . bold () , filename = filename , linenum = start . line , colnum = start . column , pipe = "|" . blue () . bold () , label = start . line . to_string () . blue () . bold () , code = code_line . trim_end () , offset = " " . repeat (start . column) , underline = "^" . repeat (end . column . saturating_sub (start . column) . max (1)) . red () . bold () , message = err . to_string () . red () ,) }
};
}
