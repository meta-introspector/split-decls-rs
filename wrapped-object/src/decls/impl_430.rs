macro_rules! deps {
    () => {
        VersionTable!();
        Version!();
        VersionIndex!();
        Note!();
        VerdefIterator!();
        SymbolIndex!();
        ReadRef!();
        Versym!();
        StringTable!();
        Endian!();
        Result!();
        FileHeader!();
        VerneedIterator!();
    };
}

macro_rules! impl_430 {
    () => {
        deps!();
        impl < 'data , Elf : FileHeader > VersionTable < 'data , Elf > { # [doc = " Parse the version sections."] pub fn parse < R : ReadRef < 'data > > (endian : Elf :: Endian , versyms : & 'data [elf :: Versym < Elf :: Endian >] , verdefs : Option < VerdefIterator < 'data , Elf > > , verneeds : Option < VerneedIterator < 'data , Elf > > , strings : StringTable < 'data , R > ,) -> Result < Self > { let mut max_index = 0 ; if let Some (mut verdefs) = verdefs . clone () { while let Some ((verdef , _)) = verdefs . next () ? { if verdef . vd_flags . get (endian) & elf :: VER_FLG_BASE != 0 { continue ; } let index = verdef . vd_ndx . get (endian) & elf :: VERSYM_VERSION ; if max_index < index { max_index = index ; } } } if let Some (mut verneeds) = verneeds . clone () { while let Some ((_ , mut vernauxs)) = verneeds . next () ? { while let Some (vernaux) = vernauxs . next () ? { let index = vernaux . vna_other . get (endian) & elf :: VERSYM_VERSION ; if max_index < index { max_index = index ; } } } } let mut versions = vec ! [Version :: default () ; max_index as usize + 1] ; if let Some (mut verdefs) = verdefs { while let Some ((verdef , mut verdauxs)) = verdefs . next () ? { if verdef . vd_flags . get (endian) & elf :: VER_FLG_BASE != 0 { continue ; } let index = verdef . vd_ndx . get (endian) & elf :: VERSYM_VERSION ; if index <= elf :: VER_NDX_GLOBAL { continue ; } if let Some (verdaux) = verdauxs . next () ? { versions [usize :: from (index)] = Version { name : verdaux . name (endian , strings) ? , hash : verdef . vd_hash . get (endian) , valid : true , file : None , } ; } } } if let Some (mut verneeds) = verneeds { while let Some ((verneed , mut vernauxs)) = verneeds . next () ? { while let Some (vernaux) = vernauxs . next () ? { let index = vernaux . vna_other . get (endian) & elf :: VERSYM_VERSION ; if index <= elf :: VER_NDX_GLOBAL { continue ; } versions [usize :: from (index)] = Version { name : vernaux . name (endian , strings) ? , hash : vernaux . vna_hash . get (endian) , valid : true , file : Some (verneed . file (endian , strings) ?) , } ; } } } Ok (VersionTable { symbols : versyms , versions , }) } # [doc = " Return true if the version table is empty."] pub fn is_empty (& self) -> bool { self . symbols . is_empty () } # [doc = " Return version index for a given symbol index."] pub fn version_index (& self , endian : Elf :: Endian , index : SymbolIndex) -> VersionIndex { let version_index = match self . symbols . get (index . 0) { Some (x) => x . 0 . get (endian) , None => elf :: VER_NDX_GLOBAL , } ; VersionIndex (version_index) } # [doc = " Return version information for a given symbol version index."] # [doc = ""] # [doc = " Returns `Ok(None)` for local and global versions."] # [doc = " Returns `Err(_)` if index is invalid."] pub fn version (& self , index : VersionIndex) -> Result < Option < & Version < 'data > > > { if index . index () <= elf :: VER_NDX_GLOBAL { return Ok (None) ; } self . versions . get (usize :: from (index . index ())) . filter (| version | version . valid) . read_error ("Invalid ELF symbol version index") . map (Some) } # [doc = " Return true if the given symbol index satisfies the requirements of `need`."] # [doc = ""] # [doc = " Returns false for any error."] # [doc = ""] # [doc = " Note: this function hasn't been fully tested and is likely to be incomplete."] pub fn matches (& self , endian : Elf :: Endian , index : SymbolIndex , need : Option < & Version < '_ > > ,) -> bool { let version_index = self . version_index (endian , index) ; let def = match self . version (version_index) { Ok (def) => def , Err (_) => return false , } ; match (def , need) { (Some (def) , Some (need)) => need . hash == def . hash && need . name == def . name , (None , Some (_need)) => { false } (Some (_def) , None) => { ! version_index . is_hidden () } (None , None) => true , } } }
    };
}

impl_430!();