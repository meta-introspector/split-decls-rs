macro_rules! is_docker {
    () => {
        # [cfg (target_os = "linux")] fn is_docker () -> bool { let has_docker_env = std :: fs :: metadata ("/.dockerenv") . is_ok () ; let has_docker_cgroup = std :: fs :: read_to_string ("/proc/self/cgroup") . map (| cgroup | cgroup . to_ascii_lowercase () . contains ("docker")) . unwrap_or (false) ; has_docker_env || has_docker_cgroup }
    };
}

is_docker!();