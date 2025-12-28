macro_rules! ZONE_INFO_DIRECTORIES {
    () => {
        # [cfg (unix)] const ZONE_INFO_DIRECTORIES : [& str ; 4] = ["/usr/share/zoneinfo" , "/share/zoneinfo" , "/etc/zoneinfo" , "/usr/share/lib/zoneinfo"] ;
    };
}

ZONE_INFO_DIRECTORIES!();