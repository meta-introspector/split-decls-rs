macro_rules! deps {
    () => {
        File!();
    };
}

macro_rules! read_file_lines {
    () => {
        deps!();
        # [track_caller] pub fn read_file_lines (path : & str) -> Vec < String > { let Ok (file) = std :: fs :: File :: open (path) else { panic ! ("failed to open file `{path}`") } ; let file = std :: io :: BufReader :: new (file) ; let mut lines = vec ! [] ; for line in file . lines () { let Ok (line) = line else { panic ! ("failed to read file lines `{path}`") ; } ; lines . push (line) ; } lines }
    };
}

read_file_lines!();