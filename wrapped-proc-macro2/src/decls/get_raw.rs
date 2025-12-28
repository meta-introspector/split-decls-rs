macro_rules! get_raw {
    () => {
        # [cfg (procmacro2_semver_exempt)] fn get_raw (repr : & str) -> Option < & str > { let pounds = repr . len () - repr . trim_start_matches ('#') . len () ; if repr . len () >= pounds + 1 + 1 + pounds && repr [pounds ..] . starts_with ('"') && repr . trim_end_matches ('#') . len () + pounds == repr . len () && repr [.. repr . len () - pounds] . ends_with ('"') { Some (& repr [pounds + 1 .. repr . len () - pounds - 1]) } else { None } }
    };
}

get_raw!()