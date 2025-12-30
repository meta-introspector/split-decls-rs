// Generated macro for tests (module)
macro_rules! Depcrate_fmt_temporal_parsertests {
() => {
// Module: crate::fmt::temporal::parser
// Provides: {"tests"}
// Dependencies: {}
# [cfg (feature = "alloc")] # [cfg (test)] mod tests { use super :: * ; # [test] fn ok_signed_duration () { let p = | input : & [u8] | { SpanParser :: new () . parse_signed_duration (input) . unwrap () } ; insta :: assert_debug_snapshot ! (p (b"PT0s") , @ "0s") ; insta :: assert_debug_snapshot ! (p (b"PT0.000000001s") , @ "1ns") ; insta :: assert_debug_snapshot ! (p (b"PT1s") , @ "1s") ; insta :: assert_debug_snapshot ! (p (b"PT59s") , @ "59s") ; insta :: assert_debug_snapshot ! (p (b"PT60s") , @ "60s") ; insta :: assert_debug_snapshot ! (p (b"PT1m") , @ "60s") ; insta :: assert_debug_snapshot ! (p (b"PT1m0.000000001s") , @ "60s 1ns") ; insta :: assert_debug_snapshot ! (p (b"PT1.25m") , @ "75s") ; insta :: assert_debug_snapshot ! (p (b"PT1h") , @ "3600s") ; insta :: assert_debug_snapshot ! (p (b"PT1h0.000000001s") , @ "3600s 1ns") ; insta :: assert_debug_snapshot ! (p (b"PT1.25h") , @ "4500s") ; insta :: assert_debug_snapshot ! (p (b"-PT2562047788015215h30m8.999999999s") , @ "-9223372036854775808s 999999999ns") ; insta :: assert_debug_snapshot ! (p (b"PT2562047788015215h30m7.999999999s") , @ "9223372036854775807s 999999999ns") ; insta :: assert_debug_snapshot ! (p (b"PT9223372036854775807S") , @ "9223372036854775807s") ; insta :: assert_debug_snapshot ! (p (b"-PT9223372036854775808S") , @ "-9223372036854775808s") ; } # [test] fn err_signed_duration () { let p = | input : & [u8] | { SpanParser :: new () . parse_signed_duration (input) . unwrap_err () } ; insta :: assert_snapshot ! (p (b"P0d") , @ r#"failed to parse "P0d" as an ISO 8601 duration string: parsing ISO 8601 duration into a `SignedDuration` requires that the duration contain a time component and no components of days or greater"# ,) ; insta :: assert_snapshot ! (p (b"PT0d") , @ r#"failed to parse "PT0d" as an ISO 8601 duration string: expected to find time unit designator suffix (H, M or S), but found "d" instead"# ,) ; insta :: assert_snapshot ! (p (b"P0dT1s") , @ r#"failed to parse "P0dT1s" as an ISO 8601 duration string: parsing ISO 8601 duration into a `SignedDuration` requires that the duration contain a time component and no components of days or greater"# ,) ; insta :: assert_snapshot ! (p (b"") , @ r#"failed to parse "" as an ISO 8601 duration string: expected to find duration beginning with 'P' or 'p', but found end of input"# ,) ; insta :: assert_snapshot ! (p (b"P") , @ r#"failed to parse "P" as an ISO 8601 duration string: parsing ISO 8601 duration into a `SignedDuration` requires that the duration contain a time component and no components of days or greater"# ,) ; insta :: assert_snapshot ! (p (b"PT") , @ r#"failed to parse "PT" as an ISO 8601 duration string: found a time designator (T or t) in an ISO 8601 duration string in "PT", but did not find any time units"# ,) ; insta :: assert_snapshot ! (p (b"PTs") , @ r#"failed to parse "PTs" as an ISO 8601 duration string: found a time designator (T or t) in an ISO 8601 duration string in "PTs", but did not find any time units"# ,) ; insta :: assert_snapshot ! (p (b"PT1s1m") , @ r#"failed to parse "PT1s1m" as an ISO 8601 duration string: found value 1 with unit minute after unit second, but units must be written from largest to smallest (and they can't be repeated)"# ,) ; insta :: assert_snapshot ! (p (b"PT1s1h") , @ r#"failed to parse "PT1s1h" as an ISO 8601 duration string: found value 1 with unit hour after unit second, but units must be written from largest to smallest (and they can't be repeated)"# ,) ; insta :: assert_snapshot ! (p (b"PT1m1h") , @ r#"failed to parse "PT1m1h" as an ISO 8601 duration string: found value 1 with unit hour after unit minute, but units must be written from largest to smallest (and they can't be repeated)"# ,) ; insta :: assert_snapshot ! (p (b"-PT9223372036854775809s") , @ r#"failed to parse "-PT9223372036854775809s" as an ISO 8601 duration string: `-9223372036854775809` seconds is too big (or small) to fit into a signed 64-bit integer"# ,) ; insta :: assert_snapshot ! (p (b"PT9223372036854775808s") , @ r#"failed to parse "PT9223372036854775808s" as an ISO 8601 duration string: `9223372036854775808` seconds is too big (or small) to fit into a signed 64-bit integer"# ,) ; insta :: assert_snapshot ! (p (b"PT1m9223372036854775807s") , @ r#"failed to parse "PT1m9223372036854775807s" as an ISO 8601 duration string: accumulated `SignedDuration` of `1m` overflowed when adding 9223372036854775807 of unit second"# ,) ; insta :: assert_snapshot ! (p (b"PT2562047788015215.6h") , @ r#"failed to parse "PT2562047788015215.6h" as an ISO 8601 duration string: accumulated `SignedDuration` of `2562047788015215h` overflowed when adding 0.600000000 of unit hour"# ,) ; } # [test] fn ok_unsigned_duration () { let p = | input : & [u8] | { SpanParser :: new () . parse_unsigned_duration (input) . unwrap () } ; insta :: assert_debug_snapshot ! (p (b"PT0s") , @ "0ns") ; insta :: assert_debug_snapshot ! (p (b"PT0.000000001s") , @ "1ns") ; insta :: assert_debug_snapshot ! (p (b"PT1s") , @ "1s") ; insta :: assert_debug_snapshot ! (p (b"+PT1s") , @ "1s") ; insta :: assert_debug_snapshot ! (p (b"PT59s") , @ "59s") ; insta :: assert_debug_snapshot ! (p (b"PT60s") , @ "60s") ; insta :: assert_debug_snapshot ! (p (b"PT1m") , @ "60s") ; insta :: assert_debug_snapshot ! (p (b"PT1m0.000000001s") , @ "60.000000001s") ; insta :: assert_debug_snapshot ! (p (b"PT1.25m") , @ "75s") ; insta :: assert_debug_snapshot ! (p (b"PT1h") , @ "3600s") ; insta :: assert_debug_snapshot ! (p (b"PT1h0.000000001s") , @ "3600.000000001s") ; insta :: assert_debug_snapshot ! (p (b"PT1.25h") , @ "4500s") ; insta :: assert_debug_snapshot ! (p (b"PT2562047788015215h30m7.999999999s") , @ "9223372036854775807.999999999s") ; insta :: assert_debug_snapshot ! (p (b"PT5124095576030431H15.999999999S") , @ "18446744073709551615.999999999s") ; insta :: assert_debug_snapshot ! (p (b"PT9223372036854775807S") , @ "9223372036854775807s") ; insta :: assert_debug_snapshot ! (p (b"PT9223372036854775808S") , @ "9223372036854775808s") ; insta :: assert_debug_snapshot ! (p (b"PT18446744073709551615S") , @ "18446744073709551615s") ; insta :: assert_debug_snapshot ! (p (b"PT1M18446744073709551555S") , @ "18446744073709551615s") ; } # [test] fn err_unsigned_duration () { # [track_caller] fn p (input : & [u8]) -> crate :: Error { SpanParser :: new () . parse_unsigned_duration (input) . unwrap_err () } insta :: assert_snapshot ! (p (b"-PT1S") , @ r#"failed to parse "-PT1S" as an ISO 8601 duration string: cannot parse negative duration into unsigned `std::time::Duration`"# ,) ; insta :: assert_snapshot ! (p (b"-PT0S") , @ r#"failed to parse "-PT0S" as an ISO 8601 duration string: cannot parse negative duration into unsigned `std::time::Duration`"# ,) ; insta :: assert_snapshot ! (p (b"P0d") , @ r#"failed to parse "P0d" as an ISO 8601 duration string: parsing ISO 8601 duration into a `SignedDuration` requires that the duration contain a time component and no components of days or greater"# ,) ; insta :: assert_snapshot ! (p (b"PT0d") , @ r#"failed to parse "PT0d" as an ISO 8601 duration string: expected to find time unit designator suffix (H, M or S), but found "d" instead"# ,) ; insta :: assert_snapshot ! (p (b"P0dT1s") , @ r#"failed to parse "P0dT1s" as an ISO 8601 duration string: parsing ISO 8601 duration into a `SignedDuration` requires that the duration contain a time component and no components of days or greater"# ,) ; insta :: assert_snapshot ! (p (b"") , @ r#"failed to parse "" as an ISO 8601 duration string: expected to find duration beginning with 'P' or 'p', but found end of input"# ,) ; insta :: assert_snapshot ! (p (b"P") , @ r#"failed to parse "P" as an ISO 8601 duration string: parsing ISO 8601 duration into a `SignedDuration` requires that the duration contain a time component and no components of days or greater"# ,) ; insta :: assert_snapshot ! (p (b"PT") , @ r#"failed to parse "PT" as an ISO 8601 duration string: found a time designator (T or t) in an ISO 8601 duration string in "PT", but did not find any time units"# ,) ; insta :: assert_snapshot ! (p (b"PTs") , @ r#"failed to parse "PTs" as an ISO 8601 duration string: found a time designator (T or t) in an ISO 8601 duration string in "PTs", but did not find any time units"# ,) ; insta :: assert_snapshot ! (p (b"PT1s1m") , @ r#"failed to parse "PT1s1m" as an ISO 8601 duration string: found value 1 with unit minute after unit second, but units must be written from largest to smallest (and they can't be repeated)"# ,) ; insta :: assert_snapshot ! (p (b"PT1s1h") , @ r#"failed to parse "PT1s1h" as an ISO 8601 duration string: found value 1 with unit hour after unit second, but units must be written from largest to smallest (and they can't be repeated)"# ,) ; insta :: assert_snapshot ! (p (b"PT1m1h") , @ r#"failed to parse "PT1m1h" as an ISO 8601 duration string: found value 1 with unit hour after unit minute, but units must be written from largest to smallest (and they can't be repeated)"# ,) ; insta :: assert_snapshot ! (p (b"-PT9223372036854775809S") , @ r#"failed to parse "-PT9223372036854775809S" as an ISO 8601 duration string: cannot parse negative duration into unsigned `std::time::Duration`"# ,) ; insta :: assert_snapshot ! (p (b"PT18446744073709551616S") , @ r#"failed to parse "PT18446744073709551616S" as an ISO 8601 duration string: number `18446744073709551616` too big to parse into 64-bit integer"# ,) ; insta :: assert_snapshot ! (p (b"PT5124095576030431H16.999999999S") , @ r#"failed to parse "PT5124095576030431H16.999999999S" as an ISO 8601 duration string: accumulated `std::time::Duration` of `18446744073709551600s` overflowed when adding 16 of unit second"# ,) ; insta :: assert_snapshot ! (p (b"PT1M18446744073709551556S") , @ r#"failed to parse "PT1M18446744073709551556S" as an ISO 8601 duration string: accumulated `std::time::Duration` of `60s` overflowed when adding 18446744073709551556 of unit second"# ,) ; insta :: assert_snapshot ! (p (b"PT5124095576030431.5H") , @ r#"failed to parse "PT5124095576030431.5H" as an ISO 8601 duration string: accumulated `std::time::Duration` of `18446744073709551600s` overflowed when adding 0.500000000 of unit hour"# ,) ; } # [test] fn ok_temporal_duration_basic () { let p = | input : & [u8] | SpanParser :: new () . parse_span (input) . unwrap () ; insta :: assert_debug_snapshot ! (p (b"P5d") , @ "5d") ; insta :: assert_debug_snapshot ! (p (b"-P5d") , @ "5d ago") ; insta :: assert_debug_snapshot ! (p (b"+P5d") , @ "5d") ; insta :: assert_debug_snapshot ! (p (b"P5DT1s") , @ "5d 1s") ; insta :: assert_debug_snapshot ! (p (b"PT1S") , @ "1s") ; insta :: assert_debug_snapshot ! (p (b"PT0S") , @ "0s") ; insta :: assert_debug_snapshot ! (p (b"P0Y") , @ "0s") ; insta :: assert_debug_snapshot ! (p (b"P1Y1M1W1DT1H1M1S") , @ "1y 1mo 1w 1d 1h 1m 1s") ; insta :: assert_debug_snapshot ! (p (b"P1y1m1w1dT1h1m1s") , @ "1y 1mo 1w 1d 1h 1m 1s") ; } # [test] fn ok_temporal_duration_fractional () { let p = | input : & [u8] | SpanParser :: new () . parse_span (input) . unwrap () ; insta :: assert_debug_snapshot ! (p (b"PT0.5h") , @ "30m") ; insta :: assert_debug_snapshot ! (p (b"PT0.123456789h") , @ "7m 24s 444ms 440µs 400ns") ; insta :: assert_debug_snapshot ! (p (b"PT1.123456789h") , @ "1h 7m 24s 444ms 440µs 400ns") ; insta :: assert_debug_snapshot ! (p (b"PT0.5m") , @ "30s") ; insta :: assert_debug_snapshot ! (p (b"PT0.123456789m") , @ "7s 407ms 407µs 340ns") ; insta :: assert_debug_snapshot ! (p (b"PT1.123456789m") , @ "1m 7s 407ms 407µs 340ns") ; insta :: assert_debug_snapshot ! (p (b"PT0.5s") , @ "500ms") ; insta :: assert_debug_snapshot ! (p (b"PT0.123456789s") , @ "123ms 456µs 789ns") ; insta :: assert_debug_snapshot ! (p (b"PT1.123456789s") , @ "1s 123ms 456µs 789ns") ; insta :: assert_debug_snapshot ! (p (b"PT1902545624836.854775807s") , @ "631107417600s 631107417600000ms 631107417600000000µs 9223372036854775807ns") ; insta :: assert_debug_snapshot ! (p (b"PT175307616h10518456960m640330789636.854775807s") , @ "175307616h 10518456960m 631107417600s 9223372036854ms 775µs 807ns") ; insta :: assert_debug_snapshot ! (p (b"-PT1902545624836.854775807s") , @ "631107417600s 631107417600000ms 631107417600000000µs 9223372036854775807ns ago") ; insta :: assert_debug_snapshot ! (p (b"-PT175307616h10518456960m640330789636.854775807s") , @ "175307616h 10518456960m 631107417600s 9223372036854ms 775µs 807ns ago") ; } # [test] fn ok_temporal_duration_unbalanced () { let p = | input : & [u8] | SpanParser :: new () . parse_span (input) . unwrap () ; insta :: assert_debug_snapshot ! (p (b"PT175307616h10518456960m1774446656760s") , @ "175307616h 10518456960m 631107417600s 631107417600000ms 512231821560000000µs") ; insta :: assert_debug_snapshot ! (p (b"Pt843517082H") , @ "175307616h 10518456960m 631107417600s 631107417600000ms 512231824800000000µs") ; insta :: assert_debug_snapshot ! (p (b"Pt843517081H") , @ "175307616h 10518456960m 631107417600s 631107417600000ms 512231821200000000µs") ; } # [test] fn ok_temporal_datetime_basic () { let p = | input | { DateTimeParser :: new () . parse_temporal_datetime (input) . unwrap () } ; insta :: assert_debug_snapshot ! (p (b"2024-06-01") , @ r###"
        Parsed {
            value: ParsedDateTime {
                input: "2024-06-01",
                date: ParsedDate {
                    input: "2024-06-01",
                    date: 2024-06-01,
                },
                time: None,
                offset: None,
                annotations: ParsedAnnotations {
                    input: "",
                    time_zone: None,
                },
            },
            input: "",
        }
        "###) ; insta :: assert_debug_snapshot ! (p (b"2024-06-01[America/New_York]") , @ r###"
        Parsed {
            value: ParsedDateTime {
                input: "2024-06-01[America/New_York]",
                date: ParsedDate {
                    input: "2024-06-01",
                    date: 2024-06-01,
                },
                time: None,
                offset: None,
                annotations: ParsedAnnotations {
                    input: "[America/New_York]",
                    time_zone: Some(
                        Named {
                            critical: false,
                            name: "America/New_York",
                        },
                    ),
                },
            },
            input: "",
        }
        "###) ; insta :: assert_debug_snapshot ! (p (b"2024-06-01T01:02:03") , @ r###"
        Parsed {
            value: ParsedDateTime {
                input: "2024-06-01T01:02:03",
                date: ParsedDate {
                    input: "2024-06-01",
                    date: 2024-06-01,
                },
                time: Some(
                    ParsedTime {
                        input: "01:02:03",
                        time: 01:02:03,
                        extended: true,
                    },
                ),
                offset: None,
                annotations: ParsedAnnotations {
                    input: "",
                    time_zone: None,
                },
            },
            input: "",
        }
        "###) ; insta :: assert_debug_snapshot ! (p (b"2024-06-01T01:02:03-05") , @ r###"
        Parsed {
            value: ParsedDateTime {
                input: "2024-06-01T01:02:03-05",
                date: ParsedDate {
                    input: "2024-06-01",
                    date: 2024-06-01,
                },
                time: Some(
                    ParsedTime {
                        input: "01:02:03",
                        time: 01:02:03,
                        extended: true,
                    },
                ),
                offset: Some(
                    ParsedOffset {
                        kind: Numeric(
                            -05,
                        ),
                    },
                ),
                annotations: ParsedAnnotations {
                    input: "",
                    time_zone: None,
                },
            },
            input: "",
        }
        "###) ; insta :: assert_debug_snapshot ! (p (b"2024-06-01T01:02:03-05[America/New_York]") , @ r###"
        Parsed {
            value: ParsedDateTime {
                input: "2024-06-01T01:02:03-05[America/New_York]",
                date: ParsedDate {
                    input: "2024-06-01",
                    date: 2024-06-01,
                },
                time: Some(
                    ParsedTime {
                        input: "01:02:03",
                        time: 01:02:03,
                        extended: true,
                    },
                ),
                offset: Some(
                    ParsedOffset {
                        kind: Numeric(
                            -05,
                        ),
                    },
                ),
                annotations: ParsedAnnotations {
                    input: "[America/New_York]",
                    time_zone: Some(
                        Named {
                            critical: false,
                            name: "America/New_York",
                        },
                    ),
                },
            },
            input: "",
        }
        "###) ; insta :: assert_debug_snapshot ! (p (b"2024-06-01T01:02:03Z[America/New_York]") , @ r###"
        Parsed {
            value: ParsedDateTime {
                input: "2024-06-01T01:02:03Z[America/New_York]",
                date: ParsedDate {
                    input: "2024-06-01",
                    date: 2024-06-01,
                },
                time: Some(
                    ParsedTime {
                        input: "01:02:03",
                        time: 01:02:03,
                        extended: true,
                    },
                ),
                offset: Some(
                    ParsedOffset {
                        kind: Zulu,
                    },
                ),
                annotations: ParsedAnnotations {
                    input: "[America/New_York]",
                    time_zone: Some(
                        Named {
                            critical: false,
                            name: "America/New_York",
                        },
                    ),
                },
            },
            input: "",
        }
        "###) ; insta :: assert_debug_snapshot ! (p (b"2024-06-01T01:02:03-01[America/New_York]") , @ r###"
        Parsed {
            value: ParsedDateTime {
                input: "2024-06-01T01:02:03-01[America/New_York]",
                date: ParsedDate {
                    input: "2024-06-01",
                    date: 2024-06-01,
                },
                time: Some(
                    ParsedTime {
                        input: "01:02:03",
                        time: 01:02:03,
                        extended: true,
                    },
                ),
                offset: Some(
                    ParsedOffset {
                        kind: Numeric(
                            -01,
                        ),
                    },
                ),
                annotations: ParsedAnnotations {
                    input: "[America/New_York]",
                    time_zone: Some(
                        Named {
                            critical: false,
                            name: "America/New_York",
                        },
                    ),
                },
            },
            input: "",
        }
        "###) ; } # [test] fn ok_temporal_datetime_incomplete () { let p = | input | { DateTimeParser :: new () . parse_temporal_datetime (input) . unwrap () } ; insta :: assert_debug_snapshot ! (p (b"2024-06-01T01") , @ r###"
        Parsed {
            value: ParsedDateTime {
                input: "2024-06-01T01",
                date: ParsedDate {
                    input: "2024-06-01",
                    date: 2024-06-01,
                },
                time: Some(
                    ParsedTime {
                        input: "01",
                        time: 01:00:00,
                        extended: false,
                    },
                ),
                offset: None,
                annotations: ParsedAnnotations {
                    input: "",
                    time_zone: None,
                },
            },
            input: "",
        }
        "###) ; insta :: assert_debug_snapshot ! (p (b"2024-06-01T0102") , @ r###"
        Parsed {
            value: ParsedDateTime {
                input: "2024-06-01T0102",
                date: ParsedDate {
                    input: "2024-06-01",
                    date: 2024-06-01,
                },
                time: Some(
                    ParsedTime {
                        input: "0102",
                        time: 01:02:00,
                        extended: false,
                    },
                ),
                offset: None,
                annotations: ParsedAnnotations {
                    input: "",
                    time_zone: None,
                },
            },
            input: "",
        }
        "###) ; insta :: assert_debug_snapshot ! (p (b"2024-06-01T01:02") , @ r###"
        Parsed {
            value: ParsedDateTime {
                input: "2024-06-01T01:02",
                date: ParsedDate {
                    input: "2024-06-01",
                    date: 2024-06-01,
                },
                time: Some(
                    ParsedTime {
                        input: "01:02",
                        time: 01:02:00,
                        extended: true,
                    },
                ),
                offset: None,
                annotations: ParsedAnnotations {
                    input: "",
                    time_zone: None,
                },
            },
            input: "",
        }
        "###) ; } # [test] fn ok_temporal_datetime_separator () { let p = | input | { DateTimeParser :: new () . parse_temporal_datetime (input) . unwrap () } ; insta :: assert_debug_snapshot ! (p (b"2024-06-01t01:02:03") , @ r###"
        Parsed {
            value: ParsedDateTime {
                input: "2024-06-01t01:02:03",
                date: ParsedDate {
                    input: "2024-06-01",
                    date: 2024-06-01,
                },
                time: Some(
                    ParsedTime {
                        input: "01:02:03",
                        time: 01:02:03,
                        extended: true,
                    },
                ),
                offset: None,
                annotations: ParsedAnnotations {
                    input: "",
                    time_zone: None,
                },
            },
            input: "",
        }
        "###) ; insta :: assert_debug_snapshot ! (p (b"2024-06-01 01:02:03") , @ r###"
        Parsed {
            value: ParsedDateTime {
                input: "2024-06-01 01:02:03",
                date: ParsedDate {
                    input: "2024-06-01",
                    date: 2024-06-01,
                },
                time: Some(
                    ParsedTime {
                        input: "01:02:03",
                        time: 01:02:03,
                        extended: true,
                    },
                ),
                offset: None,
                annotations: ParsedAnnotations {
                    input: "",
                    time_zone: None,
                },
            },
            input: "",
        }
        "###) ; } # [test] fn ok_temporal_time_basic () { let p = | input | DateTimeParser :: new () . parse_temporal_time (input) . unwrap () ; insta :: assert_debug_snapshot ! (p (b"01:02:03") , @ r###"
        Parsed {
            value: ParsedTime {
                input: "01:02:03",
                time: 01:02:03,
                extended: true,
            },
            input: "",
        }
        "###) ; insta :: assert_debug_snapshot ! (p (b"130113") , @ r###"
        Parsed {
            value: ParsedTime {
                input: "130113",
                time: 13:01:13,
                extended: false,
            },
            input: "",
        }
        "###) ; insta :: assert_debug_snapshot ! (p (b"T01:02:03") , @ r###"
        Parsed {
            value: ParsedTime {
                input: "01:02:03",
                time: 01:02:03,
                extended: true,
            },
            input: "",
        }
        "###) ; insta :: assert_debug_snapshot ! (p (b"T010203") , @ r###"
        Parsed {
            value: ParsedTime {
                input: "010203",
                time: 01:02:03,
                extended: false,
            },
            input: "",
        }
        "###) ; } # [test] fn ok_temporal_time_from_full_datetime () { let p = | input | DateTimeParser :: new () . parse_temporal_time (input) . unwrap () ; insta :: assert_debug_snapshot ! (p (b"2024-06-01T01:02:03") , @ r###"
        Parsed {
            value: ParsedTime {
                input: "01:02:03",
                time: 01:02:03,
                extended: true,
            },
            input: "",
        }
        "###) ; insta :: assert_debug_snapshot ! (p (b"2024-06-01T01:02:03.123") , @ r###"
        Parsed {
            value: ParsedTime {
                input: "01:02:03.123",
                time: 01:02:03.123,
                extended: true,
            },
            input: "",
        }
        "###) ; insta :: assert_debug_snapshot ! (p (b"2024-06-01T01") , @ r###"
        Parsed {
            value: ParsedTime {
                input: "01",
                time: 01:00:00,
                extended: false,
            },
            input: "",
        }
        "###) ; insta :: assert_debug_snapshot ! (p (b"2024-06-01T0102") , @ r###"
        Parsed {
            value: ParsedTime {
                input: "0102",
                time: 01:02:00,
                extended: false,
            },
            input: "",
        }
        "###) ; insta :: assert_debug_snapshot ! (p (b"2024-06-01T010203") , @ r###"
        Parsed {
            value: ParsedTime {
                input: "010203",
                time: 01:02:03,
                extended: false,
            },
            input: "",
        }
        "###) ; insta :: assert_debug_snapshot ! (p (b"2024-06-01T010203-05") , @ r###"
        Parsed {
            value: ParsedTime {
                input: "010203",
                time: 01:02:03,
                extended: false,
            },
            input: "",
        }
        "###) ; insta :: assert_debug_snapshot ! (p (b"2024-06-01T010203-05[America/New_York]") , @ r###"
        Parsed {
            value: ParsedTime {
                input: "010203",
                time: 01:02:03,
                extended: false,
            },
            input: "",
        }
        "###) ; insta :: assert_debug_snapshot ! (p (b"2024-06-01T010203[America/New_York]") , @ r###"
        Parsed {
            value: ParsedTime {
                input: "010203",
                time: 01:02:03,
                extended: false,
            },
            input: "",
        }
        "###) ; } # [test] fn err_temporal_time_ambiguous () { let p = | input | { DateTimeParser :: new () . parse_temporal_time (input) . unwrap_err () } ; insta :: assert_snapshot ! (p (b"010203") , @ r###"parsed time from "010203" is ambiguous with a month-day date"### ,) ; insta :: assert_snapshot ! (p (b"130112") , @ r###"parsed time from "130112" is ambiguous with a year-month date"### ,) ; } # [test] fn err_temporal_time_missing_time () { let p = | input | { DateTimeParser :: new () . parse_temporal_time (input) . unwrap_err () } ; insta :: assert_snapshot ! (p (b"2024-06-01[America/New_York]") , @ r###"successfully parsed date from "2024-06-01[America/New_York]", but no time component was found"### ,) ; insta :: assert_snapshot ! (p (b"2099-12-01[America/New_York]") , @ r###"successfully parsed date from "2099-12-01[America/New_York]", but no time component was found"### ,) ; insta :: assert_snapshot ! (p (b"2099-13-01[America/New_York]") , @ "failed to parse minute in time `2099-13-01[America/New_York]`: minute is not valid: parameter 'minute' with value 99 is not in the required range of 0..=59" ,) ; } # [test] fn err_temporal_time_zulu () { let p = | input | { DateTimeParser :: new () . parse_temporal_time (input) . unwrap_err () } ; insta :: assert_snapshot ! (p (b"T00:00:00Z") , @ "cannot parse civil time from string with a Zulu offset, parse as a `Timestamp` and convert to a civil time instead" ,) ; insta :: assert_snapshot ! (p (b"00:00:00Z") , @ "cannot parse plain time from string with a Zulu offset, parse as a `Timestamp` and convert to a plain time instead" ,) ; insta :: assert_snapshot ! (p (b"000000Z") , @ "cannot parse plain time from string with a Zulu offset, parse as a `Timestamp` and convert to a plain time instead" ,) ; insta :: assert_snapshot ! (p (b"2099-12-01T00:00:00Z") , @ "cannot parse plain time from full datetime string with a Zulu offset, parse as a `Timestamp` and convert to a plain time instead" ,) ; } # [test] fn ok_date_basic () { let p = | input | DateTimeParser :: new () . parse_date_spec (input) . unwrap () ; insta :: assert_debug_snapshot ! (p (b"2010-03-14") , @ r###"
        Parsed {
            value: ParsedDate {
                input: "2010-03-14",
                date: 2010-03-14,
            },
            input: "",
        }
        "###) ; insta :: assert_debug_snapshot ! (p (b"20100314") , @ r###"
        Parsed {
            value: ParsedDate {
                input: "20100314",
                date: 2010-03-14,
            },
            input: "",
        }
        "###) ; insta :: assert_debug_snapshot ! (p (b"2010-03-14T01:02:03") , @ r###"
        Parsed {
            value: ParsedDate {
                input: "2010-03-14",
                date: 2010-03-14,
            },
            input: "T01:02:03",
        }
        "###) ; insta :: assert_debug_snapshot ! (p (b"-009999-03-14") , @ r###"
        Parsed {
            value: ParsedDate {
                input: "-009999-03-14",
                date: -009999-03-14,
            },
            input: "",
        }
        "###) ; insta :: assert_debug_snapshot ! (p (b"+009999-03-14") , @ r###"
        Parsed {
            value: ParsedDate {
                input: "+009999-03-14",
                date: 9999-03-14,
            },
            input: "",
        }
        "###) ; } # [test] fn err_date_empty () { insta :: assert_snapshot ! (DateTimeParser :: new () . parse_date_spec (b"") . unwrap_err () , @ "failed to parse year in date ``: expected four digit year (or leading sign for six digit year), but found end of input" ,) ; } # [test] fn err_date_year () { insta :: assert_snapshot ! (DateTimeParser :: new () . parse_date_spec (b"123") . unwrap_err () , @ "failed to parse year in date `123`: expected four digit year (or leading sign for six digit year), but found end of input" ,) ; insta :: assert_snapshot ! (DateTimeParser :: new () . parse_date_spec (b"123a") . unwrap_err () , @ r#"failed to parse year in date `123a`: failed to parse "123a" as year (a four digit integer): invalid digit, expected 0-9 but got a"# ,) ; insta :: assert_snapshot ! (DateTimeParser :: new () . parse_date_spec (b"-9999") . unwrap_err () , @ "failed to parse year in date `-9999`: expected six digit year (because of a leading sign), but found end of input" ,) ; insta :: assert_snapshot ! (DateTimeParser :: new () . parse_date_spec (b"+9999") . unwrap_err () , @ "failed to parse year in date `+9999`: expected six digit year (because of a leading sign), but found end of input" ,) ; insta :: assert_snapshot ! (DateTimeParser :: new () . parse_date_spec (b"-99999") . unwrap_err () , @ "failed to parse year in date `-99999`: expected six digit year (because of a leading sign), but found end of input" ,) ; insta :: assert_snapshot ! (DateTimeParser :: new () . parse_date_spec (b"+99999") . unwrap_err () , @ "failed to parse year in date `+99999`: expected six digit year (because of a leading sign), but found end of input" ,) ; insta :: assert_snapshot ! (DateTimeParser :: new () . parse_date_spec (b"-99999a") . unwrap_err () , @ r#"failed to parse year in date `-99999a`: failed to parse "99999a" as year (a six digit integer): invalid digit, expected 0-9 but got a"# ,) ; insta :: assert_snapshot ! (DateTimeParser :: new () . parse_date_spec (b"+999999") . unwrap_err () , @ "failed to parse year in date `+999999`: year is not valid: parameter 'year' with value 999999 is not in the required range of -9999..=9999" ,) ; insta :: assert_snapshot ! (DateTimeParser :: new () . parse_date_spec (b"-010000") . unwrap_err () , @ "failed to parse year in date `-010000`: year is not valid: parameter 'year' with value 10000 is not in the required range of -9999..=9999" ,) ; } # [test] fn err_date_month () { insta :: assert_snapshot ! (DateTimeParser :: new () . parse_date_spec (b"2024-") . unwrap_err () , @ "failed to parse month in date `2024-`: expected two digit month, but found end of input" ,) ; insta :: assert_snapshot ! (DateTimeParser :: new () . parse_date_spec (b"2024") . unwrap_err () , @ "failed to parse month in date `2024`: expected two digit month, but found end of input" ,) ; insta :: assert_snapshot ! (DateTimeParser :: new () . parse_date_spec (b"2024-13-01") . unwrap_err () , @ "failed to parse month in date `2024-13-01`: month is not valid: parameter 'month' with value 13 is not in the required range of 1..=12" ,) ; insta :: assert_snapshot ! (DateTimeParser :: new () . parse_date_spec (b"20241301") . unwrap_err () , @ "failed to parse month in date `20241301`: month is not valid: parameter 'month' with value 13 is not in the required range of 1..=12" ,) ; } # [test] fn err_date_day () { insta :: assert_snapshot ! (DateTimeParser :: new () . parse_date_spec (b"2024-12-") . unwrap_err () , @ "failed to parse day in date `2024-12-`: expected two digit day, but found end of input" ,) ; insta :: assert_snapshot ! (DateTimeParser :: new () . parse_date_spec (b"202412") . unwrap_err () , @ "failed to parse day in date `202412`: expected two digit day, but found end of input" ,) ; insta :: assert_snapshot ! (DateTimeParser :: new () . parse_date_spec (b"2024-12-40") . unwrap_err () , @ "failed to parse day in date `2024-12-40`: day is not valid: parameter 'day' with value 40 is not in the required range of 1..=31" ,) ; insta :: assert_snapshot ! (DateTimeParser :: new () . parse_date_spec (b"2024-11-31") . unwrap_err () , @ "date parsed from `2024-11-31` is not valid: parameter 'day' with value 31 is not in the required range of 1..=30" ,) ; insta :: assert_snapshot ! (DateTimeParser :: new () . parse_date_spec (b"2024-02-30") . unwrap_err () , @ "date parsed from `2024-02-30` is not valid: parameter 'day' with value 30 is not in the required range of 1..=29" ,) ; insta :: assert_snapshot ! (DateTimeParser :: new () . parse_date_spec (b"2023-02-29") . unwrap_err () , @ "date parsed from `2023-02-29` is not valid: parameter 'day' with value 29 is not in the required range of 1..=28" ,) ; } # [test] fn err_date_separator () { insta :: assert_snapshot ! (DateTimeParser :: new () . parse_date_spec (b"2024-1231") . unwrap_err () , @ "failed to parse separator after month: expected `-` separator, but found `3` instead" ,) ; insta :: assert_snapshot ! (DateTimeParser :: new () . parse_date_spec (b"202412-31") . unwrap_err () , @ "failed to parse separator after month: expected no separator after month since none was found after the year, but found a `-` separator" ,) ; } # [test] fn ok_time_basic () { let p = | input | DateTimeParser :: new () . parse_time_spec (input) . unwrap () ; insta :: assert_debug_snapshot ! (p (b"01:02:03") , @ r###"
        Parsed {
            value: ParsedTime {
                input: "01:02:03",
                time: 01:02:03,
                extended: true,
            },
            input: "",
        }
        "###) ; insta :: assert_debug_snapshot ! (p (b"010203") , @ r###"
        Parsed {
            value: ParsedTime {
                input: "010203",
                time: 01:02:03,
                extended: false,
            },
            input: "",
        }
        "###) ; } # [test] fn ok_time_fractional () { let p = | input | DateTimeParser :: new () . parse_time_spec (input) . unwrap () ; insta :: assert_debug_snapshot ! (p (b"01:02:03.123456789") , @ r###"
        Parsed {
            value: ParsedTime {
                input: "01:02:03.123456789",
                time: 01:02:03.123456789,
                extended: true,
            },
            input: "",
        }
        "###) ; insta :: assert_debug_snapshot ! (p (b"010203.123456789") , @ r###"
        Parsed {
            value: ParsedTime {
                input: "010203.123456789",
                time: 01:02:03.123456789,
                extended: false,
            },
            input: "",
        }
        "###) ; insta :: assert_debug_snapshot ! (p (b"01:02:03.9") , @ r###"
        Parsed {
            value: ParsedTime {
                input: "01:02:03.9",
                time: 01:02:03.9,
                extended: true,
            },
            input: "",
        }
        "###) ; } # [test] fn ok_time_no_fractional () { let p = | input | DateTimeParser :: new () . parse_time_spec (input) . unwrap () ; insta :: assert_debug_snapshot ! (p (b"01:02.123456789") , @ r###"
        Parsed {
            value: ParsedTime {
                input: "01:02",
                time: 01:02:00,
                extended: true,
            },
            input: ".123456789",
        }
        "###) ; } # [test] fn ok_time_leap () { let p = | input | DateTimeParser :: new () . parse_time_spec (input) . unwrap () ; insta :: assert_debug_snapshot ! (p (b"01:02:60") , @ r###"
        Parsed {
            value: ParsedTime {
                input: "01:02:60",
                time: 01:02:59,
                extended: true,
            },
            input: "",
        }
        "###) ; } # [test] fn ok_time_mixed_format () { let p = | input | DateTimeParser :: new () . parse_time_spec (input) . unwrap () ; insta :: assert_debug_snapshot ! (p (b"01:0203") , @ r###"
        Parsed {
            value: ParsedTime {
                input: "01:02",
                time: 01:02:00,
                extended: true,
            },
            input: "03",
        }
        "###) ; insta :: assert_debug_snapshot ! (p (b"0102:03") , @ r###"
        Parsed {
            value: ParsedTime {
                input: "0102",
                time: 01:02:00,
                extended: false,
            },
            input: ":03",
        }
        "###) ; } # [test] fn err_time_empty () { insta :: assert_snapshot ! (DateTimeParser :: new () . parse_time_spec (b"") . unwrap_err () , @ "failed to parse hour in time ``: expected two digit hour, but found end of input" ,) ; } # [test] fn err_time_hour () { insta :: assert_snapshot ! (DateTimeParser :: new () . parse_time_spec (b"a") . unwrap_err () , @ "failed to parse hour in time `a`: expected two digit hour, but found end of input" ,) ; insta :: assert_snapshot ! (DateTimeParser :: new () . parse_time_spec (b"1a") . unwrap_err () , @ r#"failed to parse hour in time `1a`: failed to parse "1a" as hour (a two digit integer): invalid digit, expected 0-9 but got a"# ,) ; insta :: assert_snapshot ! (DateTimeParser :: new () . parse_time_spec (b"24") . unwrap_err () , @ "failed to parse hour in time `24`: hour is not valid: parameter 'hour' with value 24 is not in the required range of 0..=23" ,) ; } # [test] fn err_time_minute () { insta :: assert_snapshot ! (DateTimeParser :: new () . parse_time_spec (b"01:") . unwrap_err () , @ "failed to parse minute in time `01:`: expected two digit minute, but found end of input" ,) ; insta :: assert_snapshot ! (DateTimeParser :: new () . parse_time_spec (b"01:a") . unwrap_err () , @ "failed to parse minute in time `01:a`: expected two digit minute, but found end of input" ,) ; insta :: assert_snapshot ! (DateTimeParser :: new () . parse_time_spec (b"01:1a") . unwrap_err () , @ "failed to parse minute in time `01:1a`: failed to parse `1a` as minute (a two digit integer): invalid digit, expected 0-9 but got a" ,) ; insta :: assert_snapshot ! (DateTimeParser :: new () . parse_time_spec (b"01:60") . unwrap_err () , @ "failed to parse minute in time `01:60`: minute is not valid: parameter 'minute' with value 60 is not in the required range of 0..=59" ,) ; } # [test] fn err_time_second () { insta :: assert_snapshot ! (DateTimeParser :: new () . parse_time_spec (b"01:02:") . unwrap_err () , @ "failed to parse second in time `01:02:`: expected two digit second, but found end of input" ,) ; insta :: assert_snapshot ! (DateTimeParser :: new () . parse_time_spec (b"01:02:a") . unwrap_err () , @ "failed to parse second in time `01:02:a`: expected two digit second, but found end of input" ,) ; insta :: assert_snapshot ! (DateTimeParser :: new () . parse_time_spec (b"01:02:1a") . unwrap_err () , @ "failed to parse second in time `01:02:1a`: failed to parse `1a` as second (a two digit integer): invalid digit, expected 0-9 but got a" ,) ; insta :: assert_snapshot ! (DateTimeParser :: new () . parse_time_spec (b"01:02:61") . unwrap_err () , @ "failed to parse second in time `01:02:61`: second is not valid: parameter 'second' with value 61 is not in the required range of 0..=59" ,) ; } # [test] fn err_time_fractional () { insta :: assert_snapshot ! (DateTimeParser :: new () . parse_time_spec (b"01:02:03.") . unwrap_err () , @ "failed to parse fractional nanoseconds in time `01:02:03.`: found decimal after seconds component, but did not find any decimal digits after decimal" ,) ; insta :: assert_snapshot ! (DateTimeParser :: new () . parse_time_spec (b"01:02:03.a") . unwrap_err () , @ "failed to parse fractional nanoseconds in time `01:02:03.a`: found decimal after seconds component, but did not find any decimal digits after decimal" ,) ; } # [test] fn ok_iso_week_date_parse_basic () { fn p (input : & str) -> Parsed < '_ , ISOWeekDate > { DateTimeParser :: new () . parse_iso_week_date (input . as_bytes ()) . unwrap () } insta :: assert_debug_snapshot ! (p ("2024-W01-5") , @ r#"
        Parsed {
            value: ISOWeekDate {
                year: 2024,
                week: 1,
                weekday: Friday,
            },
            input: "",
        }
        "#) ; insta :: assert_debug_snapshot ! (p ("2024-W52-7") , @ r#"
        Parsed {
            value: ISOWeekDate {
                year: 2024,
                week: 52,
                weekday: Sunday,
            },
            input: "",
        }
        "#) ; insta :: assert_debug_snapshot ! (p ("2004-W53-6") , @ r#"
        Parsed {
            value: ISOWeekDate {
                year: 2004,
                week: 53,
                weekday: Saturday,
            },
            input: "",
        }
        "#) ; insta :: assert_debug_snapshot ! (p ("2009-W01-1") , @ r#"
        Parsed {
            value: ISOWeekDate {
                year: 2009,
                week: 1,
                weekday: Monday,
            },
            input: "",
        }
        "#) ; insta :: assert_debug_snapshot ! (p ("2024W015") , @ r#"
        Parsed {
            value: ISOWeekDate {
                year: 2024,
                week: 1,
                weekday: Friday,
            },
            input: "",
        }
        "#) ; insta :: assert_debug_snapshot ! (p ("2024W527") , @ r#"
        Parsed {
            value: ISOWeekDate {
                year: 2024,
                week: 52,
                weekday: Sunday,
            },
            input: "",
        }
        "#) ; insta :: assert_debug_snapshot ! (p ("2004W536") , @ r#"
        Parsed {
            value: ISOWeekDate {
                year: 2004,
                week: 53,
                weekday: Saturday,
            },
            input: "",
        }
        "#) ; insta :: assert_debug_snapshot ! (p ("2009W011") , @ r#"
        Parsed {
            value: ISOWeekDate {
                year: 2009,
                week: 1,
                weekday: Monday,
            },
            input: "",
        }
        "#) ; insta :: assert_debug_snapshot ! (p ("2009w011") , @ r#"
        Parsed {
            value: ISOWeekDate {
                year: 2009,
                week: 1,
                weekday: Monday,
            },
            input: "",
        }
        "#) ; } # [test] fn err_iso_week_date_year () { let p = | input : & str | { DateTimeParser :: new () . parse_iso_week_date (input . as_bytes ()) . unwrap_err () } ; insta :: assert_snapshot ! (p ("123") , @ "failed to parse year in date `123`: expected four digit year (or leading sign for six digit year), but found end of input" ,) ; insta :: assert_snapshot ! (p ("123a") , @ r#"failed to parse year in date `123a`: failed to parse "123a" as year (a four digit integer): invalid digit, expected 0-9 but got a"# ,) ; insta :: assert_snapshot ! (p ("-9999") , @ "failed to parse year in date `-9999`: expected six digit year (because of a leading sign), but found end of input" ,) ; insta :: assert_snapshot ! (p ("+9999") , @ "failed to parse year in date `+9999`: expected six digit year (because of a leading sign), but found end of input" ,) ; insta :: assert_snapshot ! (p ("-99999") , @ "failed to parse year in date `-99999`: expected six digit year (because of a leading sign), but found end of input" ,) ; insta :: assert_snapshot ! (p ("+99999") , @ "failed to parse year in date `+99999`: expected six digit year (because of a leading sign), but found end of input" ,) ; insta :: assert_snapshot ! (p ("-99999a") , @ r#"failed to parse year in date `-99999a`: failed to parse "99999a" as year (a six digit integer): invalid digit, expected 0-9 but got a"# ,) ; insta :: assert_snapshot ! (p ("+999999") , @ "failed to parse year in date `+999999`: year is not valid: parameter 'year' with value 999999 is not in the required range of -9999..=9999" ,) ; insta :: assert_snapshot ! (p ("-010000") , @ "failed to parse year in date `-010000`: year is not valid: parameter 'year' with value 10000 is not in the required range of -9999..=9999" ,) ; } # [test] fn err_iso_week_date_week_prefix () { let p = | input : & str | { DateTimeParser :: new () . parse_iso_week_date (input . as_bytes ()) . unwrap_err () } ; insta :: assert_snapshot ! (p ("2024-") , @ "failed to parse week number prefix in date `2024-`: expected `W` or `w`, but found end of input" ,) ; insta :: assert_snapshot ! (p ("2024") , @ "failed to parse week number prefix in date `2024`: expected `W` or `w`, but found end of input" ,) ; } # [test] fn err_iso_week_date_week_number () { let p = | input : & str | { DateTimeParser :: new () . parse_iso_week_date (input . as_bytes ()) . unwrap_err () } ; insta :: assert_snapshot ! (p ("2024-W") , @ "failed to parse week number in date `2024-W`: expected two digit week number, but found end of input" ,) ; insta :: assert_snapshot ! (p ("2024-W1") , @ "failed to parse week number in date `2024-W1`: expected two digit week number, but found end of input" ,) ; insta :: assert_snapshot ! (p ("2024-W53-1") , @ "week date parsed from `2024-W53-1` is not valid: ISO week number `53` is invalid for year `2024`" ,) ; insta :: assert_snapshot ! (p ("2030W531") , @ "week date parsed from `2030W531` is not valid: ISO week number `53` is invalid for year `2030`" ,) ; } # [test] fn err_iso_week_date_parse_incomplete () { let p = | input : & str | { DateTimeParser :: new () . parse_iso_week_date (input . as_bytes ()) . unwrap_err () } ; insta :: assert_snapshot ! (p ("2024-W53-1") , @ "week date parsed from `2024-W53-1` is not valid: ISO week number `53` is invalid for year `2024`" ,) ; insta :: assert_snapshot ! (p ("2025-W53-1") , @ "week date parsed from `2025-W53-1` is not valid: ISO week number `53` is invalid for year `2025`" ,) ; } # [test] fn err_iso_week_date_date_day () { let p = | input : & str | { DateTimeParser :: new () . parse_iso_week_date (input . as_bytes ()) . unwrap_err () } ; insta :: assert_snapshot ! (p ("2024-W12-") , @ "failed to parse weekday in date `2024-W12-`: expected one digit weekday, but found end of input" ,) ; insta :: assert_snapshot ! (p ("2024W12") , @ "failed to parse weekday in date `2024W12`: expected one digit weekday, but found end of input" ,) ; insta :: assert_snapshot ! (p ("2024-W11-8") , @ "failed to parse weekday in date `2024-W11-8`: parsed weekday `8` is not valid: parameter 'weekday' with value 8 is not in the required range of 1..=7" ,) ; } # [test] fn err_iso_week_date_date_separator () { let p = | input : & str | { DateTimeParser :: new () . parse_iso_week_date (input . as_bytes ()) . unwrap_err () } ; insta :: assert_snapshot ! (p ("2024-W521") , @ "failed to parse separator after week number: expected `-` separator, but found `1` instead" ,) ; insta :: assert_snapshot ! (p ("2024W01-5") , @ "failed to parse separator after week number: expected no separator after month since none was found after the year, but found a `-` separator" ,) ; } }
};
}
