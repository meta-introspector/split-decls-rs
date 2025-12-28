macro_rules! on_left_angle_typed {
    () => {
        # [doc = " Add closing `>` for generic arguments/parameters."] fn on_left_angle_typed (file : & SourceFile , reparsed : & SourceFile , offset : TextSize ,) -> Option < TextEdit > { let file_text = reparsed . syntax () . text () ; let mut next_offset = offset ; while file_text . char_at (next_offset) == Some (' ') { next_offset += TextSize :: of (' ') } if file_text . char_at (next_offset) == Some ('>') { return None ; } if ancestors_at_offset (file . syntax () , offset) . take_while (| n | ! ast :: Item :: can_cast (n . kind ())) . any (| n | { ast :: GenericParamList :: can_cast (n . kind ()) || ast :: GenericArgList :: can_cast (n . kind ()) || ast :: UseBoundGenericArgs :: can_cast (n . kind ()) }) { Some (TextEdit :: insert (offset + TextSize :: of ('<') , '>' . to_string ())) } else { None } }
    };
}

on_left_angle_typed!();