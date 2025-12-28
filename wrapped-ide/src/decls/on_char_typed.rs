macro_rules! on_char_typed {
    () => {
        pub (crate) fn on_char_typed (db : & RootDatabase , position : FilePosition , char_typed : char ,) -> Option < SourceChange > { if ! TRIGGER_CHARS . contains (& char_typed) { return None ; } let edition = Edition :: CURRENT_FIXME ; let editioned_file_id_wrapper = EditionedFileId :: new (db , position . file_id , edition) ; let file = & db . parse (editioned_file_id_wrapper) ; let char_matches_position = file . tree () . syntax () . text () . char_at (position . offset) == Some (char_typed) ; if ! stdx :: always ! (char_matches_position) { return None ; } let edit = on_char_typed_ (file , position . offset , char_typed , edition) ? ; let mut sc = SourceChange :: from_text_edit (position . file_id , edit . edit) ; sc . is_snippet = edit . is_snippet ; Some (sc) }
    };
}

on_char_typed!()