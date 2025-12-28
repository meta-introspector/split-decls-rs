macro_rules! use_panic_2021 {
    () => {
        pub (crate) fn use_panic_2021 (mut span : Span) -> bool { loop { let expn = span . ctxt () . outer_expn_data () ; if let Some (features) = expn . allow_internal_unstable && features . contains (& sym :: edition_panic) { span = expn . call_site ; continue ; } break expn . edition >= Edition :: Edition2021 ; } }
    };
}

use_panic_2021!();