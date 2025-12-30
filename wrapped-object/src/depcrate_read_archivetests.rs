// Generated macro for tests (module)
macro_rules! Depcrate_read_archivetests {
() => {
// Module: crate::read::archive
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn kind () { let data = b"!<arch>\n" ; let archive = ArchiveFile :: parse (& data [..]) . unwrap () ; assert_eq ! (archive . kind () , ArchiveKind :: Unknown) ; let data = b"\
            !<arch>\n\
            /                                               4         `\n\
            0000" ; let archive = ArchiveFile :: parse (& data [..]) . unwrap () ; assert_eq ! (archive . kind () , ArchiveKind :: Gnu) ; let data = b"\
            !<arch>\n\
            //                                              4         `\n\
            0000" ; let archive = ArchiveFile :: parse (& data [..]) . unwrap () ; assert_eq ! (archive . kind () , ArchiveKind :: Gnu) ; let data = b"\
            !<arch>\n\
            /                                               4         `\n\
            0000\
            //                                              4         `\n\
            0000" ; let archive = ArchiveFile :: parse (& data [..]) . unwrap () ; assert_eq ! (archive . kind () , ArchiveKind :: Gnu) ; let data = b"\
            !<arch>\n\
            /SYM64/                                         4         `\n\
            0000" ; let archive = ArchiveFile :: parse (& data [..]) . unwrap () ; assert_eq ! (archive . kind () , ArchiveKind :: Gnu64) ; let data = b"\
            !<arch>\n\
            /SYM64/                                         4         `\n\
            0000\
            //                                              4         `\n\
            0000" ; let archive = ArchiveFile :: parse (& data [..]) . unwrap () ; assert_eq ! (archive . kind () , ArchiveKind :: Gnu64) ; let data = b"\
            !<arch>\n\
            __.SYMDEF                                       4         `\n\
            0000" ; let archive = ArchiveFile :: parse (& data [..]) . unwrap () ; assert_eq ! (archive . kind () , ArchiveKind :: Bsd) ; let data = b"\
            !<arch>\n\
            #1/9                                            13        `\n\
            __.SYMDEF0000" ; let archive = ArchiveFile :: parse (& data [..]) . unwrap () ; assert_eq ! (archive . kind () , ArchiveKind :: Bsd) ; let data = b"\
            !<arch>\n\
            #1/16                                           20        `\n\
            __.SYMDEF SORTED0000" ; let archive = ArchiveFile :: parse (& data [..]) . unwrap () ; assert_eq ! (archive . kind () , ArchiveKind :: Bsd) ; let data = b"\
            !<arch>\n\
            __.SYMDEF_64                                    4         `\n\
            0000" ; let archive = ArchiveFile :: parse (& data [..]) . unwrap () ; assert_eq ! (archive . kind () , ArchiveKind :: Bsd64) ; let data = b"\
            !<arch>\n\
            #1/12                                           16        `\n\
            __.SYMDEF_640000" ; let archive = ArchiveFile :: parse (& data [..]) . unwrap () ; assert_eq ! (archive . kind () , ArchiveKind :: Bsd64) ; let data = b"\
            !<arch>\n\
            #1/19                                           23        `\n\
            __.SYMDEF_64 SORTED0000" ; let archive = ArchiveFile :: parse (& data [..]) . unwrap () ; assert_eq ! (archive . kind () , ArchiveKind :: Bsd64) ; let data = b"\
            !<arch>\n\
            /                                               4         `\n\
            0000\
            /                                               4         `\n\
            0000\
            //                                              4         `\n\
            0000" ; let archive = ArchiveFile :: parse (& data [..]) . unwrap () ; assert_eq ! (archive . kind () , ArchiveKind :: Coff) ; let data = b"\
            <bigaf>\n\
            0                   0                   \
            0                   0                   \
            0                   128                 \
            6                   0                   \
            0                   \0\0\0\0\0\0\0\0\0\0\0\0\
            \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\
            \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\
            \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0" ; let archive = ArchiveFile :: parse (& data [..]) . unwrap () ; assert_eq ! (archive . kind () , ArchiveKind :: AixBig) ; let data = b"\
            !<thin>\n\
            /                                               4         `\n\
            0000" ; let archive = ArchiveFile :: parse (& data [..]) . unwrap () ; assert_eq ! (archive . kind () , ArchiveKind :: Gnu) ; assert ! (archive . is_thin ()) ; } # [test] fn gnu_names () { let data = b"\
            !<arch>\n\
            //                                              18        `\n\
            0123456789abcdef/\n\
            s p a c e/      0           0     0     644     4         `\n\
            0000\
            0123456789abcde/0           0     0     644     3         `\n\
            odd\n\
            /0              0           0     0     644     4         `\n\
            even" ; let data = & data [..] ; let archive = ArchiveFile :: parse (data) . unwrap () ; assert_eq ! (archive . kind () , ArchiveKind :: Gnu) ; let mut members = archive . members () ; let member = members . next () . unwrap () . unwrap () ; assert_eq ! (member . name () , b"s p a c e") ; assert_eq ! (member . data (data) . unwrap () , & b"0000" [..]) ; let member = members . next () . unwrap () . unwrap () ; assert_eq ! (member . name () , b"0123456789abcde") ; assert_eq ! (member . data (data) . unwrap () , & b"odd" [..]) ; let member = members . next () . unwrap () . unwrap () ; assert_eq ! (member . name () , b"0123456789abcdef") ; assert_eq ! (member . data (data) . unwrap () , & b"even" [..]) ; assert ! (members . next () . is_none ()) ; } # [test] fn thin_gnu_names () { let data = b"\
            !<thin>\n\
            //                                              18        `\n\
            0123456789/abcde/\n\
            s p a c e/      0           0     0     644     4         `\n\
            0123456789abcde/0           0     0     644     3         `\n\
            /0              0           0     0     644     4         `\n\
            " ; let data = & data [..] ; let archive = ArchiveFile :: parse (data) . unwrap () ; assert_eq ! (archive . kind () , ArchiveKind :: Gnu) ; let mut members = archive . members () ; let member = members . next () . unwrap () . unwrap () ; assert_eq ! (member . name () , b"s p a c e") ; assert ! (member . is_thin ()) ; assert_eq ! (member . size () , 4) ; assert_eq ! (member . data (data) . unwrap () , & []) ; let member = members . next () . unwrap () . unwrap () ; assert_eq ! (member . name () , b"0123456789abcde") ; assert ! (member . is_thin ()) ; assert_eq ! (member . size () , 3) ; assert_eq ! (member . data (data) . unwrap () , & []) ; let member = members . next () . unwrap () . unwrap () ; assert_eq ! (member . name () , b"0123456789/abcde") ; assert ! (member . is_thin ()) ; assert_eq ! (member . size () , 4) ; assert_eq ! (member . data (data) . unwrap () , & []) ; assert ! (members . next () . is_none ()) ; } # [test] fn bsd_names () { let data = b"\
            !<arch>\n\
            0123456789abcde 0           0     0     644     3         `\n\
            odd\n\
            #1/16           0           0     0     644     20        `\n\
            0123456789abcdefeven" ; let data = & data [..] ; let archive = ArchiveFile :: parse (data) . unwrap () ; assert_eq ! (archive . kind () , ArchiveKind :: Unknown) ; let mut members = archive . members () ; let member = members . next () . unwrap () . unwrap () ; assert_eq ! (member . name () , b"0123456789abcde") ; assert_eq ! (member . data (data) . unwrap () , & b"odd" [..]) ; let member = members . next () . unwrap () . unwrap () ; assert_eq ! (member . name () , b"0123456789abcdef") ; assert_eq ! (member . data (data) . unwrap () , & b"even" [..]) ; assert ! (members . next () . is_none ()) ; } # [test] fn aix_names () { let data = b"\
            <bigaf>\n\
            396                 0                   0                   \
            128                 262                 0                   \
            4                   262                 0                   \
            1662610370  223         1           644         16  \
            0123456789abcdef`\nord\n\
            4                   396                 128                 \
            1662610374  223         1           644         16  \
            fedcba9876543210`\nrev\n\
            94                  0                   262                 \
            0           0           0           0           0   \
            `\n2                   128                 \
            262                 0123456789abcdef\0fedcba9876543210\0" ; let data = & data [..] ; let archive = ArchiveFile :: parse (data) . unwrap () ; assert_eq ! (archive . kind () , ArchiveKind :: AixBig) ; let mut members = archive . members () ; let member = members . next () . unwrap () . unwrap () ; assert_eq ! (member . name () , b"0123456789abcdef") ; assert_eq ! (member . data (data) . unwrap () , & b"ord\n" [..]) ; let member = members . next () . unwrap () . unwrap () ; assert_eq ! (member . name () , b"fedcba9876543210") ; assert_eq ! (member . data (data) . unwrap () , & b"rev\n" [..]) ; assert ! (members . next () . is_none ()) ; } }
};
}
