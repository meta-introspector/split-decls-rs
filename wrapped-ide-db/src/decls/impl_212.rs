macro_rules! deps {
    () => {
        SymbolIndex!();
        Query!();
    };
}

macro_rules! impl_212 {
    () => {
        deps!();
        impl Query { pub (crate) fn search < 'sym , T > (self , indices : & 'sym [& SymbolIndex] , cb : impl FnMut (& 'sym FileSymbol) -> ControlFlow < T > ,) -> Option < T > { let _p = tracing :: info_span ! ("symbol_index::Query::search") . entered () ; let mut op = fst :: map :: OpBuilder :: new () ; match self . mode { SearchMode :: Exact => { let automaton = fst :: automaton :: Str :: new (& self . lowercased) ; for index in indices . iter () { op = op . add (index . map . search (& automaton)) ; } self . search_maps (indices , op . union () , cb) } SearchMode :: Fuzzy => { let automaton = fst :: automaton :: Subsequence :: new (& self . lowercased) ; for index in indices . iter () { op = op . add (index . map . search (& automaton)) ; } self . search_maps (indices , op . union () , cb) } SearchMode :: Prefix => { let automaton = fst :: automaton :: Str :: new (& self . lowercased) . starts_with () ; for index in indices . iter () { op = op . add (index . map . search (& automaton)) ; } self . search_maps (indices , op . union () , cb) } } } fn search_maps < 'sym , T > (& self , indices : & 'sym [& SymbolIndex] , mut stream : fst :: map :: Union < '_ > , mut cb : impl FnMut (& 'sym FileSymbol) -> ControlFlow < T > ,) -> Option < T > { let ignore_underscore_prefixed = ! self . query . starts_with ("__") ; while let Some ((_ , indexed_values)) = stream . next () { for & IndexedValue { index , value } in indexed_values { let symbol_index = & indices [index] ; let (start , end) = SymbolIndex :: map_value_to_range (value) ; for symbol in & symbol_index . symbols [start .. end] { let non_type_for_type_only_query = self . only_types && ! matches ! (symbol . def , hir :: ModuleDef :: Adt (..) | hir :: ModuleDef :: TypeAlias (..) | hir :: ModuleDef :: BuiltinType (..) | hir :: ModuleDef :: Trait (..)) ; if non_type_for_type_only_query || ! self . matches_assoc_mode (symbol . is_assoc) { continue ; } let symbol_name = symbol . name . as_str () ; if ignore_underscore_prefixed && symbol_name . starts_with ("__") { continue ; } if self . exclude_imports && symbol . is_import { continue ; } if self . mode . check (& self . query , self . case_sensitive , symbol_name) && let Some (b) = cb (symbol) . break_value () { return Some (b) ; } } } } None } fn matches_assoc_mode (& self , is_trait_assoc_item : bool) -> bool { ! matches ! ((is_trait_assoc_item , self . assoc_mode) , (true , AssocSearchMode :: Exclude) | (false , AssocSearchMode :: AssocItemsOnly)) } }
    };
}

impl_212!()