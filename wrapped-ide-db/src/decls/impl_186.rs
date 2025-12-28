macro_rules! deps {
    () => {
        SnippetEdit!();
        Snippet!();
    };
}

macro_rules! impl_186 {
    () => {
        deps!();
        impl SnippetEdit { pub fn new (snippets : Vec < Snippet >) -> Self { let mut snippet_ranges = snippets . into_iter () . zip (1 ..) . with_position () . flat_map (| pos | { let (snippet , index) = match pos { (itertools :: Position :: First , it) | (itertools :: Position :: Middle , it) => it , (itertools :: Position :: Last , (snippet , _)) | (itertools :: Position :: Only , (snippet , _)) => (snippet , 0) , } ; match snippet { Snippet :: Tabstop (pos) => vec ! [(index , TextRange :: empty (pos))] , Snippet :: Placeholder (range) => vec ! [(index , range)] , Snippet :: PlaceholderGroup (ranges) => { ranges . into_iter () . map (| range | (index , range)) . collect () } } }) . collect_vec () ; snippet_ranges . sort_by_key (| (_ , range) | range . start ()) ; let disjoint_ranges = snippet_ranges . iter () . zip (snippet_ranges . iter () . skip (1)) . all (| ((_ , left) , (_ , right)) | left . end () <= right . start () || left == right) ; stdx :: always ! (disjoint_ranges) ; SnippetEdit (snippet_ranges) } # [doc = " Inserts all of the snippets into the given text."] pub fn apply (& self , text : & mut String) { for (index , range) in self . 0 . iter () . rev () { if range . is_empty () { text . insert_str (range . start () . into () , & format ! ("${index}")) ; } else { text . insert (range . end () . into () , '}') ; text . insert_str (range . start () . into () , & format ! ("${{{index}:")) ; } } } # [doc = " Gets the underlying snippet index + text range"] # [doc = " Tabstops are represented by an empty range, and placeholders use the range that they were given"] pub fn into_edit_ranges (self) -> Vec < (u32 , TextRange) > { self . 0 } }
    };
}

impl_186!();