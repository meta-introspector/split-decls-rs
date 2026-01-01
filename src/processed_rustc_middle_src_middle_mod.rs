// SRC: ../rust/compiler/rustc_middle/src/middle/mod.rs
/* AST_META: AST_ID=1 | TYPE=FUNCTION | NAME=LibFeatures | COMPLEXITY=10 | LINES=32 */
pub mod lib_features {
    use crate::rustc_data_structures::unord::UnordMap;
    use rustc_macros::{HashStable, TyDecodable, TyEncodable};
    use crate::rustc_complete::{Span, Symbol};

    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    #[derive(HashStable, TyEncodable, TyDecodable)]
    pub enum FeatureStability {
        AcceptedSince(Symbol),
        Unstable { old_name: Option<Symbol> },
    }

    #[derive(HashStable, Debug, Default)]
    pub struct LibFeatures {
        pub stability: UnordMap<Symbol, (FeatureStability, Span)>,
    }

    impl LibFeatures {
        pub fn to_sorted_vec(&self) -> Vec<(Symbol, FeatureStability)> {
            self.stability
                .to_sorted_stable_ord()
                .iter()
                .map(|&(&sym, &(stab, _))| (sym, stab))
                .collect()
        }
    }
}
/* AST_META: AST_ID=2 | TYPE=MODULE | NAME=UNNAMED | COMPLEXITY=1 | LINES=4 */