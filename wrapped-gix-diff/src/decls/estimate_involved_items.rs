macro_rules! deps {
    () => {
        ChangeKind!();
    };
}

macro_rules! estimate_involved_items {
    () => {
        deps!();
        # [cfg (test)] mod estimate_involved_items { use super :: estimate_involved_items ; use crate :: rewrites :: tracker :: { visit :: SourceKind , ChangeKind } ; # [test] fn renames_count_unemitted_as_sources_and_destinations () { let items = [(false , ChangeKind :: Addition) , (true , ChangeKind :: Deletion) , (true , ChangeKind :: Deletion) ,] ; assert_eq ! (estimate_involved_items (items , SourceKind :: Rename) , (0 , 1) , "here we only have one eligible source, hence nothing to do") ; assert_eq ! (estimate_involved_items (items . into_iter () . map (| t | (false , t . 1)) , SourceKind :: Rename) , (2 , 1) , "now we have more possibilities as renames count un-emitted deletions as source") ; } # [test] fn copies_do_not_count_additions_as_sources () { let items = [(false , ChangeKind :: Addition) , (true , ChangeKind :: Addition) , (true , ChangeKind :: Deletion) ,] ; assert_eq ! (estimate_involved_items (items , SourceKind :: Copy) , (0 , 1) , "one addition as source, the other isn't counted as it's emitted, nor is it considered a copy-source.\
            deletions don't count") ; } # [test] fn copies_count_modifications_as_sources () { let items = [(false , ChangeKind :: Addition) , (true , ChangeKind :: Modification) , (false , ChangeKind :: Modification) ,] ; assert_eq ! (estimate_involved_items (items , SourceKind :: Copy) , (2 , 1) , "any modifications is a valid source, emitted or not") ; } }
    };
}

estimate_involved_items!();